package entity

import (
	"blog-backend/cache"
	"blog-backend/logger"
	"blog-backend/model/conn"
	"blog-backend/utils/crypt"
	"encoding/json"
	"fmt"
	"github.com/dgrijalva/jwt-go"
	"time"
)

var (
	jwtSecret = []byte("xxxs")
	Expire    = 7 * 24 * time.Hour
)

func (u *User) Authenticate() error {
	// password base64
	var user User
	password := crypt.Sha256(u.Password)
	err := conn.MysqlConn.Model(User{}).Where("user_name=? and password=?", u.UserName, password).Update("last_login", time.Now()).First(&user).Error
	if err != nil {
		return err
	}
	return nil
}

func GetUsersByIDs(ids []int) ([]User, error) {
	users := []User{}
	if err := conn.MysqlConn.Model(User{}).Where("id in (?)", ids).Find(&users).Error; err != nil {
		logger.Logger.Error(fmt.Sprintf("GetUsersByIDs error: %v", err.Error()))
		return nil, err
	}
	return users, nil
}

// Claims Claim是一些实体（通常指的用户）的状态和额外的元数据
type Claims struct {
	UserInfo User
	jwt.StandardClaims
}

// GenerateToken 根据用户的用户名和密码产生token
func (u User) GenerateToken() (string, error) {
	//设置token有效时间
	nowTime := time.Now()
	expireTime := nowTime.Add(Expire)
	claims := Claims{
		UserInfo: u,
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

func (u *User) GetOrCreate() {

}