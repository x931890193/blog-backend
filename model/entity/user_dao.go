package entity

import (
	"blog-backend/cache"
	"blog-backend/logger"
	"blog-backend/model/conn"
	"blog-backend/utils/crypt"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/dgrijalva/jwt-go"
	"gorm.io/gorm"
	"time"
)

var (
	jwtSecret = []byte("xxxs")
	Expire    = 7 * 24 * time.Hour
)

func (u *User) Authenticate() error {
	// password base64
	password := crypt.Sha256(u.Password)
	err := conn.MysqlConn.Model(u).Where("user_name=? and password=?", u.UserName, password).Update("last_login", time.Now()).First(u).Error
	if err != nil {
		return err
	}
	return nil
}

func GetUsersByIDs(ids []int) ([]User, error) {
	users := []User{}
	if len(ids) == 0 {
		return users, nil
	}
	if err := conn.MysqlConn.Model(User{}).Where("id in (?)", ids).Find(&users).Error; err != nil {
		logger.Logger.Error(fmt.Sprintf("GetUsersByIDs error: %v", err.Error()))
		return nil, err
	}
	return users, nil
}

// Claims Claim是一些实体（通常指的用户）的状态和额外的元数据
type Claims struct {
	UserInfo User
	BaseInfo BaseModel
	jwt.StandardClaims
}

// GenerateToken 根据用户的用户名和密码产生token
func (u User) GenerateToken() (string, error) {
	//设置token有效时间
	nowTime := time.Now()
	expireTime := nowTime.Add(Expire)
	claims := Claims{
		UserInfo: u,
		BaseInfo: u.BaseModel,
		StandardClaims: jwt.StandardClaims{
			// 过期时间
			ExpiresAt: expireTime.Unix(),
			// 指定token发行人
			Issuer: "sato-blog",
		},
	}

	tokenClaims := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	//该方法内部生成签名字符串，再用于获取完整、已签名的token
	token, err := tokenClaims.SignedString(jwtSecret)
	userInfoByte, _ := json.Marshal(u)
	cache.Client.Set(token, string(userInfoByte), 7*24*time.Hour)
	return token, err
}

// ParseToken 根据传入的token值获取到Claims对象信息，（进而获取其中的用户名和密码）
func ParseToken(token string) (*Claims, error) {

	//用于解析鉴权的声明，方法内部主要是具体的解码和校验的过程，最终返回*Token
	tokenClaims, err := jwt.ParseWithClaims(token, &Claims{}, func(token *jwt.Token) (interface{}, error) {
		return jwtSecret, nil
	})

	if tokenClaims != nil {
		// 从tokenClaims中获取到Claims对象，并使用断言，将该对象转换为我们自己定义的Claims
		// 要传入指针，项目中结构体都是用指针传递，节省空间。
		if claims, ok := tokenClaims.Claims.(*Claims); ok && tokenClaims.Valid {
			return claims, nil
		}
	}
	return nil, err
}

func (u *User) GetOrCreate() error {
	toUpdate := map[string]interface{}{
		"last_login": time.Now(),
		"avatar":     u.Avatar,
		"email":      u.Email,
		"github_url": u.GitHubUrl,
	}
	if err := conn.MysqlConn.Model(u).Where("github_id=?", u.GitHubId).FirstOrCreate(u).UpdateColumns(toUpdate).Error; err != nil {
		return err
	}
	return nil
}

func (u *User) Update(v map[string]interface{}) error {
	if err := conn.MysqlConn.Model(u).Where("id=?", u.ID).UpdateColumns(v).Find(u).Error; err != nil {
		return err
	}
	return nil
}

func (u *User) GetListByQuery(v map[string]interface{}) ([]*User, error) {
	users := []*User{}
	if err := conn.MysqlConn.Model(u).Where(v).Find(&users).Error; err != nil {
		return nil, err
	}
	return users, nil
}

func AdminExists() (bool, error) {
	var count int64
	if err := conn.MysqlConn.Model(&User{}).Where("is_admin = ?", true).Count(&count).Error; err != nil {
		return false, err
	}
	return count > 0, nil
}

func CreateBootstrapAdmin(username, passwordHash string) (*User, error) {
	now := time.Now()
	user := &User{}
	err := conn.MysqlConn.Where("user_name = ?", username).First(user).Error
	if err == nil {
		updates := map[string]interface{}{
			"password":       passwordHash,
			"is_admin":       true,
			"receive_update": true,
			"last_login":     now,
			"updated_at":     now,
		}
		if user.Avatar == "" {
			updates["avatar"] = "https://www.bytealien.com/favicon.ico"
		}
		if user.Email == "" {
			updates["email"] = username + "@bytealien.local"
		}
		if err := conn.MysqlConn.Model(user).UpdateColumns(updates).First(user).Error; err != nil {
			return nil, err
		}
		return user, nil
	}
	if !errors.Is(err, gorm.ErrRecordNotFound) {
		return nil, err
	}

	user = &User{
		UserName:      username,
		Password:      passwordHash,
		Avatar:        "https://www.bytealien.com/favicon.ico",
		Label:         0,
		Email:         username + "@bytealien.local",
		GitHubId:      0,
		GitHubUrl:     "",
		IsAdmin:       true,
		ReceiveUpdate: true,
		LastLogin:     now,
	}
	if err := conn.MysqlConn.Create(user).Error; err != nil {
		return nil, err
	}
	return user, nil
}

func GetAdminUsers(pageSize, currentPage int, username, phone, status, beginTime, endTime string) ([]*User, int64, error) {
	users := []*User{}
	var total int64
	db := conn.MysqlConn.Model(&User{}).Where("is_delete = ?", false)
	if username != "" {
		db = db.Where("user_name like ?", "%"+username+"%")
	}
	if phone != "" {
		db = db.Where("phone like ?", "%"+phone+"%")
	}
	switch status {
	case "0", "true", "admin":
		db = db.Where("is_admin = ?", true)
	case "1", "false", "user":
		db = db.Where("is_admin = ?", false)
	}
	if beginTime != "" {
		db = db.Where("created_at >= ?", beginTime+" 00:00:00")
	}
	if endTime != "" {
		db = db.Where("created_at <= ?", endTime+" 23:59:59")
	}
	if err := db.Count(&total).Error; err != nil {
		return nil, 0, err
	}
	if pageSize > 0 && currentPage > 0 {
		db = db.Limit(pageSize).Offset((currentPage - 1) * pageSize)
	}
	if err := db.Order("id desc").Find(&users).Error; err != nil {
		return nil, 0, err
	}
	return users, total, nil
}

func GetAdminUserByID(id int) (*User, error) {
	user := &User{}
	if err := conn.MysqlConn.Model(user).Where("id = ? and is_delete = ?", id, false).First(user).Error; err != nil {
		return nil, err
	}
	return user, nil
}

func CreateAdminUser(user *User) error {
	return conn.MysqlConn.Create(user).Error
}

func UpdateAdminUserByID(id int, values map[string]interface{}) (*User, error) {
	user := &User{}
	if err := conn.MysqlConn.Model(user).Where("id = ?", id).UpdateColumns(values).First(user, id).Error; err != nil {
		return nil, err
	}
	return user, nil
}

func DeleteAdminUsersByIds(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Model(&User{}).Where("id in (?)", ids).Update("is_delete", true).Error
}
