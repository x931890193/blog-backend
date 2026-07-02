package github

import (
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/utils/http"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	nethttp "net/http"
	"net/url"
	"strconv"
	"strings"
	"time"
)

const (
	gitHubAccessTokenUrl = "https://github.com/login/oauth/access_token"
	githubUserInfoUrl    = "https://api.github.com/user"
)

type accessTokenGetResp struct {
	AccessToken      string `json:"access_token"`
	Scope            string `json:"scope"`
	TokenType        string `json:"token_type"`
	RedirectUri      string `json:"redirect_uri"`
	Error            string `json:"error"`
	ErrorDescription string `json:"error_description"`
	ErrorUri         string `json:"error_uri"`
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
	code = strings.TrimSpace(code)
	if code == "" {
		return nil, errors.New("github oauth code is empty")
	}
	params := url.Values{}
	params.Set("client_id", config.Cfg.GitHub.ClientId)
	params.Set("client_secret", config.Cfg.GitHub.ClientSecret)
	params.Set("code", code)
	params.Set("redirect_uri", RedirectURI())
	res, err := postGitHubForm(gitHubAccessTokenUrl, params)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("GetAccessToken: %s", err.Error()))
		return nil, err
	}
	tokenRes := accessTokenGetResp{}
	err = json.Unmarshal(res, &tokenRes)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("GetAccessToken: %s", err.Error()))
		return nil, err
	}
	if strings.TrimSpace(tokenRes.AccessToken) == "" {
		if tokenRes.Error != "" {
			return nil, fmt.Errorf("github oauth token exchange failed: %s %s", tokenRes.Error, tokenRes.ErrorDescription)
		}
		return nil, errors.New("github oauth token exchange failed: empty access_token")
	}
	return &tokenRes, nil
}

func postGitHubForm(endpoint string, params url.Values) ([]byte, error) {
	req, err := nethttp.NewRequest("POST", endpoint, strings.NewReader(params.Encode()))
	if err != nil {
		return nil, err
	}
	req.Header.Add("Accept", "application/json")
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	req.Header.Add("User-Agent", utils.UserAgent)
	client := nethttp.Client{Timeout: utils.RequestTimeOut}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, errors.New("github oauth token exchange status " + strconv.Itoa(resp.StatusCode) + ": " + truncateBody(body))
	}
	return body, nil
}

func truncateBody(body []byte) string {
	content := strings.TrimSpace(string(body))
	runes := []rune(content)
	if len(runes) > 240 {
		return string(runes[:240]) + "..."
	}
	return content
}

func RedirectURI() string {
	if config.Cfg != nil && config.Cfg.GitHub.RedirectUri != "" {
		return config.Cfg.GitHub.RedirectUri
	}
	return config.Host + "/api/user/github/oauth"
}

func GetUserInfo(token string) (*User, error) {
	token = strings.TrimSpace(token)
	if token == "" {
		return nil, errors.New("github access_token is empty")
	}
	res, err := utils.Get(githubUserInfoUrl, nil, utils.ContentTypeJson, map[string]string{
		"Accept":        "application/json",
		"Authorization": fmt.Sprintf("Bearer %v", token),
	})
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("GetUserInfo: %s", err.Error()))
		return nil, err
	}
	githubUser := &User{}
	err = json.Unmarshal(res, githubUser)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("GetUserInfo: %s", err.Error()))
		return nil, err
	}
	return githubUser, nil
}
