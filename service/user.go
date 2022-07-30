package service

import "blog-backend/model/entity"

func Auth(username, password string) (token string, err error) {
	user := entity.User{UserName: username, Password: password}
	err = user.Authenticate()
	if err != nil {
		return "", err
	}
	token, err = user.GenerateToken()
	if err != nil {
		return "", err
	}
	return token, nil
}

func AdminInfo(token string) (*entity.User, error) {
	parseToken, err := entity.ParseToken(token)
	if err != nil {
		return nil, err
	}
	userInfo := parseToken.UserInfo
	return &userInfo, err

}
