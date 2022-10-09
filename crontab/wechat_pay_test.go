package crontab

import "testing"

func TestGetFlow(t *testing.T) {
	GetFlow(1, 100)
}

func TestGetUsername(t *testing.T) {
	_, err := getUsername(3206623700662617344)
	if err != nil {
		return
	}
}

func TestSaveWechatOrder(t *testing.T) {
	SaveWechatOrder()
}
