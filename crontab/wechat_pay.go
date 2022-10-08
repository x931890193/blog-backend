package crontab

import (
	"blog-backend/cache"
	"blog-backend/config"
	"blog-backend/logger"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"time"
)

type tokenRes struct {
	AccessToken string `json:"access_token"`
	ExpiresIn   int    `json:"expires_in"`
}

type flowReq struct {
	PageNum  int `json:"page_num"`
	PageSize int `json:"page_size"`
}

type flowItem struct {
	OrderId       int64  `json:"order_id"`
	Status        string `json:"status"`
	Price         int    `json:"price"`
	OrderPrice    int    `json:"order_price"`
	DiscountFee   int    `json:"discount_fee"`
	ServiceFee    int    `json:"service_fee"`
	HandlingFee   int    `json:"handling_fee"`
	RefundFee     int    `json:"refund_fee"`
	SettledTime   int    `json:"settled_time"`
	OrderTime     int    `json:"order_time"`
	RefundingType int    `json:"refunding_type"`
}
type flowResp struct {
	OrderFlow []*flowItem `json:"order_flow"`
	TotalNum  int         `json:"total_num"`
	TotalPage int         `json:"total_page"`
}

func getAccessToken() (string, error) {
	tokenCache, err := cache.Client.Get("access_token").Result()
	if tokenCache != "" {
		return tokenCache, nil
	}
	url := fmt.Sprintf("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=%s&secret=%s",
		config.Cfg.WechatPay.AppID, config.Cfg.WechatPay.AppSecret)
	req, _ := http.NewRequest("GET", url, nil)
	client := http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		logger.Logger.Error(err.Error())
		return "", err
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			logger.Logger.Error(err.Error())
		}
	}(resp.Body)
	if resp.StatusCode != 200 {
		logger.Logger.Error(err.Error())
		return "", err
	}
	body, _ := ioutil.ReadAll(resp.Body)
	res := tokenRes{}
	err = json.Unmarshal(body, &res)
	if err != nil {
		logger.Logger.Error(err.Error())
		return "", err
	}
	err = cache.Client.Set("access_token", res.AccessToken, time.Second*time.Duration(res.ExpiresIn)).Err()
	if err != nil {
		return "", err
	}
	return res.AccessToken, nil
}

func GetFlow(pageNum, PageSize int) {
	token, err := getAccessToken()
	if err != nil {
		return
	}
	reqData := flowReq{pageNum, PageSize}
	url := fmt.Sprintf("https://api.weixin.qq.com/product/funds/scanorderflow?access_token=%s", token)
	reqJson, _ := json.Marshal(reqData)
	r := bytes.NewReader(reqJson)
	req, _ := http.NewRequest("POST", url, r)
	req.Header.Add("User-Agent", "application/json")
	client := http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			logger.Logger.Error(err.Error())
		}
	}(resp.Body)
	if resp.StatusCode != 200 {
		return
	}
	flowresp := flowResp{}
	body, _ := ioutil.ReadAll(resp.Body)
	err = json.Unmarshal(body, &flowresp)
	if err != nil {
		return
	}
	fmt.Println(flowresp.OrderFlow[0], flowresp.OrderFlow[1])
}
