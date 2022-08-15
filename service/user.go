package service

import (
	"blog-backend/model/entity"
	"blog-backend/utils"
	"blog-backend/utils/github"
	"errors"
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

func ParseToken(token string) (*entity.User, error) {
	parseToken, err := entity.ParseToken(token)
	if err != nil {
		return nil, err
	}
	userInfo := parseToken.UserInfo
	return &userInfo, err

}

func NewTempUser() *entity.User {
	user := &entity.User{}
	user.UserName = "游客"
	user.ID = -1
	user.Label = utils.GetRandomTag()
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
	obj := &entity.User{
		GitHubId:  user.Id,
		UserName:  user.Name,
		Avatar:    user.AvatarUrl,
		Email:     user.Email,
		GitHubUrl: user.Url,
		LastLogin: time.Now(),
	}
	err := obj.GetOrCreate()
	if err != nil {
		return nil, err
	}

	return obj, nil
}
