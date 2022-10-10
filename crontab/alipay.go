package crontab

import (
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/model/entity"
	"fmt"
	"github.com/smartwalle/alipay/v3"
	"strconv"
	"time"
)

var client *alipay.Client

func init() {
	c, err := alipay.New(config.Cfg.AliPay.AppId, config.Cfg.AliPay.PrivateKey, true)
	if err != nil {
		logger.Logger.Error(err.Error())
		panic(err)
	}
	err = c.LoadAliPayPublicKey(config.Cfg.AliPay.PublicKey)
	if err != nil {
		logger.Logger.Error(err.Error())
		panic(err)
	}
	client = c
}

func GetAccountBalance() {
	balance, err := client.BillBalanceQuery(alipay.BillBalanceQuery{})
	if err != nil {
		logger.Logger.Error(err.Error() + " query")
		return
	}
	fmt.Println("total :", balance.Content.TotalAmount)
}

func GetAccountLog(startTime, endTime string) ([]*alipay.AccountLogItem, error) {
	balance, err := client.BillAccountLogQuery(alipay.BillAccountLogQuery{StartTime: startTime, EndTime: endTime})
	if err != nil {
		logger.Logger.Error(err.Error() + " query")
		return nil, err
	}
	return balance.Content.DetailList, nil
}

func SaveAliOrder() {
	now := time.Now()
	lastTime := now.Add(-time.Hour * 48)
	res, err := GetAccountLog(lastTime.Format("2006-01-02 15:04:05"), now.Format("2006-01-02 15:04:05"))
	if err != nil {
		logger.Logger.Error(err.Error())
	}
	rewards := []entity.Reward{}
	for _, r := range res {
		amount, err := strconv.ParseFloat(r.TransAmount, 64)
		if err != nil {
			logger.Logger.Error(err.Error())
			continue
		}
		if amount < 0 {
			continue
		}
		var local, _ = time.LoadLocation("Asia/Shanghai")
		timeCreate, err := time.ParseInLocation("2006-01-02 15:04:05", r.TransDt, local)
		if err != nil {
			continue
		}
		rewards = append(rewards, entity.Reward{
			BaseModel:     entity.BaseModel{CreatedAt: timeCreate},
			OrderId:       r.AliPayOrderNo,
			Who:           r.OtherAccount,
			Amount:        amount,
			PaymentMethod: entity.Ali,
		})
	}
	obj := entity.Reward{}
	obj.InsertMany(rewards)
}
