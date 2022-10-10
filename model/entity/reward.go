package entity

import (
	"blog-backend/logger"
	"blog-backend/model/conn"
	"gorm.io/gorm/clause"
)

const (
	Ali    = 0
	WeChat = 1
)

func (r *Reward) GetList() ([]Reward, error) {
	res := []Reward{}
	if err := conn.MysqlConn.Model(r).Find(&res).Order("create_at Desc").Error; err != nil {
		return nil, err
	}
	return res, nil
}

func (r *Reward) InsertMany(orders []Reward) {
	// insert ignore into
	db := conn.MysqlConn.Clauses(clause.Insert{Modifier: "IGNORE"})
	if err := db.Model(&Reward{}).CreateInBatches(orders, len(orders)).Error; err != nil {
		logger.Logger.Error(err.Error())
	}
}

func (r *Reward) GetPayMethodHuman() string {
	switch r.PaymentMethod {
	case Ali:
		return "AliPay"
	case WeChat:
		return "WeChatPay"
	default:
		return "Unknown"
	}
}
