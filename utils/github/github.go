package github

import (
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/utils/http"
	"encoding/json"
	"fmt"
	"time"
)

const (
	gitHubAccessTokenUrl = "https://github.com/login/oauth/access_token"
	githubUserInfoUrl    = "https://api.github.com/user"
)

type accessTokenGetRequest struct {
	ClientId     string `json:"client_id"`
	ClientSecret string `json:"client_secret"`
	Code         string `json:"code"`
	RedirectUri  string `json:"redirect_uri"`
}

type accessTokenGetResp struct {
	//access_token=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&scope=&token_type=bearer
	AccessToken string `json:"access_token"`
	Scope       string `json:"scope"`
	TokenType   string `json:"token_type"`
	RedirectUri string `json:"redirect_uri"`
}

type User struct {
	Login             string      `json:"login"`
	Id                int         `json:"id"`
	NodeId            string      `json:"node_id"`
	AvatarUrl         string      `json:"avatar_url"`
	GravatarId        string      `json:"gravatar_id"`
	Url               string      `json:"url"`
	HtmlUrl           string      `json:"html_url"`
	FollowersUrl      string      `json:"followers_url"`
	FollowingUrl      string      `json:"following_url"`
	GistsUrl          string      `json:"gists_url"`
	StarredUrl        string      `json:"starred_url"`
	SubscriptionsUrl  string      `json:"subscriptions_url"`
	OrganizationsUrl  string      `json:"organizations_url"`
	ReposUrl          string      `json:"repos_url"`
	EventsUrl         string      `json:"events_url"`
	ReceivedEventsUrl string      `json:"received_events_url"`
	Type              string      `json:"type"`
	SiteAdmin         bool        `json:"site_admin"`
	Name              string      `json:"name"`
	Company           interface{} `json:"company"`
	Blog              string      `json:"blog"`
	Location          string      `json:"location"`
	Email             string      `json:"email"`
	Hireable          bool        `json:"hireable"`
	Bio               string      `json:"bio"`
	TwitterUsername   interface{} `json:"twitter_username"`
	PublicRepos       int         `json:"public_repos"`
	PublicGists       int         `json:"public_gists"`
	Followers         int         `json:"followers"`
	Following         int         `json:"following"`
	CreatedAt         time.Time   `json:"created_at"`
	UpdatedAt         time.Time   `json:"updated_at"`
}

func GetAccessToken(code string) (*accessTokenGetResp, error) {
	params := map[string]string{
		"client_id":     config.Cfg.Github.ClientId,
		"client_secret": config.Cfg.Github.ClientSecret,
		"code":          code,
		"redirect_uri":  "",
	}
	urlParameters := gitHubAccessTokenUrl + "?"
	for k, v := range params {
		urlParameters += fmt.Sprintf("%v=%v&", k, v)
	}
	res, err := utils.Post(urlParameters, nil, utils.ContentTypeJson, map[string]string{"Accept": "application/json"})
	if err != nil {
		logger.Logger.Error("GetAccessToken: ", err.Error())
		return nil, err
	}
	tokenRes := accessTokenGetResp{}
	err = json.Unmarshal(res, &tokenRes)
	if err != nil {
		logger.Logger.Error("GetAccessToken: ", err.Error())
		return nil, err
	}
	return &tokenRes, nil
}

func GetUserInfo(token string) (*User, error) {
	res, err := utils.Get(githubUserInfoUrl, nil, utils.ContentTypeJson, map[string]string{"Authorization": fmt.Sprintf("Bearer %v", token)})
	if err != nil {
		logger.Logger.Error("GetUserInfo: ", err.Error())
		return nil, err
	}
	githubUser := &User{}
	err = json.Unmarshal(res, githubUser)
	if err != nil {
		logger.Logger.Error("GetUserInfo: ", err.Error())
		return nil, err
	}
	return githubUser, nil
}
