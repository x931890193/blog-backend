package crontab

import (
	"fmt"
	"testing"
)

func TestGetFlow(t *testing.T) {
	GetFlow(1, 500)
}

func TestGetUsername(t *testing.T) {
	name, err := getUsername(3210457716765493248)
	fmt.Println(111, name)
	if err != nil {
		return
	}
}

func TestSaveWechatOrder(t *testing.T) {
	SaveWechatOrder()
}

func TestGetUserInfo(t *testing.T) {
	res, _ := GetUserInfo("ocTxB5S4dlbQYJtwn_9lMDUtwKaU")
	fmt.Println(res)
}
