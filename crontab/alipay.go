package crontab

import (
	"blog-backend/config"
	"blog-backend/logger"
	"fmt"
	"github.com/smartwalle/alipay/v3"
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
		logger.Logger.Error(err.Error(), " query")
		return
	}
	fmt.Println("total :", balance.Content.TotalAmount)
}

func GetAccountLog() {
	balance, err := client.BillAccountLogQuery(alipay.BillAccountLogQuery{StartTime: "2022-09-29 00:00:00", EndTime: "2022-10-1 00:00:00"})
	if err != nil {
		logger.Logger.Error(err.Error(), " query")
		return
	}
	fmt.Println("total :", balance.Content.DetailList[0])
}
