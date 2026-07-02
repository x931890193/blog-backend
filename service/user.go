package service

import (
	"blog-backend/config"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"blog-backend/utils/crypt"
	"blog-backend/utils/github"
	"encoding/base64"
	"errors"
	"fmt"
	"math/rand"
	"strings"
	"time"
)

func UserAuth(username, password string) (token string, err error) {
	user := entity.User{UserName: username, Password: password}
	err = user.Authenticate()
	if err != nil {
		return "", err
	}
	if !user.IsAdmin {
		return "", errors.New("你不是管理员哦～～～")
	}
	token, err = user.GenerateToken()
	if err != nil {
		return "", err
	}
	return token, nil
}

func GenerateAdmin(username, password string) (*entity.User, error) {
	username = strings.TrimSpace(username)
	password = strings.TrimSpace(password)
	if username == "" {
		username = "admin"
	}
	if password == "" {
		return nil, errors.New("管理员密码不能为空")
	}
	adminExists, err := entity.AdminExists()
	if err != nil {
		return nil, err
	}
	if adminExists {
		return nil, errors.New("管理员已存在，禁止再次初始化")
	}
	encodedPassword := base64.StdEncoding.EncodeToString([]byte(password))
	return entity.CreateBootstrapAdmin(username, crypt.Sha256(encodedPassword))
}

func ParseToken(token string) (*entity.User, error) {
	parseToken, err := entity.ParseToken(token)
	if err != nil {
		return nil, err
	}
	userInfo := parseToken.UserInfo
	userInfo.BaseModel = parseToken.BaseInfo
	return &userInfo, err

}

func NewTempUser() *entity.User {
	user := &entity.User{}
	user.UserName = "游客"
	user.ID = -1
	user.Label = rand.Intn(len(config.UserTags))
	return user
}

func GetUsersByIds(ids []int) ([]entity.User, error) {
	users, err := entity.GetUsersByIDs(ids)
	if err != nil {
		return nil, err
	}
	return users, nil
}

func GetUsersMapByIds(ids []int) (map[int]entity.User, error) {
	userMap := map[int]entity.User{}
	users, err := entity.GetUsersByIDs(ids)
	if err != nil {
		return nil, err
	}
	for _, user := range users {
		userMap[user.ID] = user
	}
	return userMap, nil
}

func GetOrCreateGitHubUser(user *github.User) (*entity.User, error) {
	username := strings.TrimSpace(user.Name)
	if username == "" {
		username = strings.TrimSpace(user.Login)
	}
	if username == "" {
		username = fmt.Sprintf("github_%d", user.Id)
	}
	email := strings.TrimSpace(user.Email)
	if email == "" {
		email = fmt.Sprintf("%d@github.local", user.Id)
	}
	githubURL := strings.TrimSpace(user.HtmlUrl)
	if githubURL == "" {
		githubURL = strings.TrimSpace(user.Url)
	}
	obj := &entity.User{
		GitHubId:  user.Id,
		UserName:  username,
		Avatar:    user.AvatarUrl,
		Email:     email,
		GitHubUrl: githubURL,
		LastLogin: time.Now(),
	}
	err := obj.GetOrCreate()
	if err != nil {
		return nil, err
	}

	return obj, nil
}

func UpdateUser(req *pb.EditUserInfoRequest) (*entity.User, error) {
	user := entity.User{}
	user.ID = int(req.UserId)
	err := user.Update(map[string]interface{}{
		"label":          req.Label,
		"receive_update": req.ReceiveUpdate,
	})
	if err != nil {
		return nil, err
	}
	return &user, nil
}
