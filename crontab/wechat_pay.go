package crontab

import (
	"blog-backend/cache"
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/model/entity"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"strconv"
	"time"
)

const (
	accessToken = "access_token"
	timeOut     = time.Second * 5
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

type address struct {
	UserName string `json:"user_name"`
}

type addressInfo struct {
	AddressInfo address `json:"address_info"`
}

type deliveryInfo struct {
	DeliveryInfo addressInfo `json:"delivery_info"`
}

type orderDetail struct {
	OrderDetail deliveryInfo `json:"order_detail"`
}

type orderResp struct {
	Order orderDetail `json:"order"`
}

type orderReq struct {
	OrderId int64 `json:"order_id"`
}

func getAccessToken() (string, error) {
	tokenCache, err := cache.Client.Get(accessToken).Result()
	if tokenCache != "" {
		return tokenCache, nil
	}
	url := fmt.Sprintf("https://api.weixin.qq.com/cgi-bin/stable_token")
	reqBody, _ := json.Marshal(map[string]string{
		"grant_type": "client_credential",
		"appid":      config.Cfg.WechatPay.AppID,
		"secret":     config.Cfg.WechatPay.AppSecret,
	})
	r := bytes.NewReader(reqBody)
	req, _ := http.NewRequest("POST", url, r)
	client := http.Client{Timeout: timeOut}
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
	err = cache.Client.Set(accessToken, res.AccessToken, time.Second*time.Duration(res.ExpiresIn)).Err()
	if err != nil {
		return "", err
	}
	return res.AccessToken, nil
}

func getUsername(orderId int64) (string, error) {
	token, err := getAccessToken()
	if err != nil {
		return "", err
	}
	reqData := orderReq{orderId}
	reqJson, _ := json.Marshal(reqData)
	r := bytes.NewReader(reqJson)
	url := fmt.Sprintf("https://api.weixin.qq.com/product/order/get?access_token=%s", token)
	req, _ := http.NewRequest("POST", url, r)
	client := http.Client{Timeout: timeOut}
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
	res := orderResp{}
	err = json.Unmarshal(body, &res)
	if err != nil {
		logger.Logger.Error(err.Error())
		return "", err
	}
	return res.Order.OrderDetail.DeliveryInfo.AddressInfo.UserName, nil
}

type UserInfo struct {
	Errcode  int    `json:"errcode"`
	Errmsg   string `json:"errmsg"`
	UnionId  string `json:"union_id"`
	NickName string `json:"nick_name"`
	Mobile   string `json:"mobile"`
	Birthday string `json:"birthday"`
	Avatar   string `json:"avatar"`
	Email    string `json:"email"`
	Sex      int    `json:"sex"`
	Country  string `json:"country"`
	AuthAt   int64  `json:"auth_at"`
	Openid   string `json:"openid"`
}

func GetUserInfo(openid string) (*UserInfo, error) {
	// noqa
	token, err := getAccessToken()
	if err != nil {
		return nil, err
	}
	url := fmt.Sprintf("https://api.weixin.qq.com/wxa/servicemarket/connector/shop/userinfo/get?access_token=%s", token)
	reqJson, _ := json.Marshal(map[string]string{
		"openid": openid,
	})
	r := bytes.NewReader(reqJson)
	req, _ := http.NewRequest("POST", url, r)
	req.Header.Add("User-Agent", "application/json")
	c := http.Client{Timeout: timeOut}
	resp, err := c.Do(req)
	if err != nil {
		return nil, err
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			logger.Logger.Error(err.Error())
		}
	}(resp.Body)
	if resp.StatusCode != 200 {
		return nil, err
	}
	user := UserInfo{}
	body, _ := ioutil.ReadAll(resp.Body)
	err = json.Unmarshal(body, &user)
	if err != nil {
		return nil, err
	}
	return &user, err
}

func GetFlow(pageNum, PageSize int) (*flowResp, error) {
	token, err := getAccessToken()
	if err != nil {
		return nil, err
	}
	reqData := flowReq{pageNum, PageSize}
	url := fmt.Sprintf("https://api.weixin.qq.com/product/funds/scanorderflow?access_token=%s", token)
	reqJson, _ := json.Marshal(reqData)
	r := bytes.NewReader(reqJson)
	req, _ := http.NewRequest("POST", url, r)
	req.Header.Add("User-Agent", "application/json")
	c := http.Client{Timeout: timeOut}
	resp, err := c.Do(req)
	if err != nil {
		return nil, err
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			logger.Logger.Error(err.Error())
		}
	}(resp.Body)
	if resp.StatusCode != 200 {
		return nil, err
	}
	flowresp := flowResp{}
	body, _ := ioutil.ReadAll(resp.Body)
	err = json.Unmarshal(body, &flowresp)
	if err != nil {
		return nil, err
	}
	return &flowresp, err
}

func SaveWechatOrder() {
	rewards := []entity.Reward{}
	pageNum := 1
	PageSize := 100
	for {
		res, err := GetFlow(pageNum, PageSize)
		pageNum++
		if len(res.OrderFlow) == 0 {
			break
		}
		if err != nil {
			logger.Logger.Error(err.Error())
			continue
		}
		for _, r := range res.OrderFlow {
			if r.Status != "ORDER_FLOW_STATUS_TRANSACTION_SUC" {
				continue
			}
			amount := float64(r.OrderPrice) / 100
			username, err := getUsername(r.OrderId)
			if err != nil {
				logger.Logger.Error(err.Error())
				continue
			}
			rewards = append(rewards, entity.Reward{
				BaseModel:     entity.BaseModel{CreatedAt: time.Unix(int64(r.OrderTime), 0)},
				OrderId:       strconv.FormatInt(r.OrderId, 10),
				Who:           username,
				Amount:        amount,
				PaymentMethod: entity.WeChat,
			})
		}
	}
	obj := entity.Reward{}
	obj.InsertMany(rewards)
}
