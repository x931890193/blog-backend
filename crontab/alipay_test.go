package crontab

import "testing"

func TestAli(t *testing.T) {
	GetAccountLog()
}

func TestGetUsername(t *testing.T) {
	_, err := getUsername(3206623700662617344)
	if err != nil {
		return
	}
}
