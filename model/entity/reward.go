package entity

import (
	"blog-backend/logger"
	"blog-backend/model/conn"
	"gorm.io/gorm"
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

// Create insert ignore into
func (r *Reward) Create(db *gorm.DB) error {
	return db.Clauses(clause.Insert{Modifier: "IGNORE"}).Create(r).Error
}

func (r *Reward) InsertMany(orders []Reward) {
	if err := conn.MysqlConn.Model(&Reward{}).CreateInBatches(orders, len(orders)).Error; err != nil {
		logger.Logger.Error(err)
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
