package entity

import (
	"blog-backend/model/conn"
)

const (
	VerifyInt = iota
	VerifyPass
	VerifyFailed
)

func (l *Link) AddOne() error {
	if err := conn.MysqlConn.Model(l).Create(&l).Error; err != nil {
		return err
	}
	return nil
}

func (l *Link) GetAllList() ([]*Link, error) {
	res := []*Link{}
	if err := conn.MysqlConn.Model(l).Find(&res).Error; err != nil {
		return nil, err
	}
	return res, nil
}

func (l *Link) GetLinByUserId() error {
	if err := conn.MysqlConn.Model(l).Find(l).Error; err != nil {
		return err
	}
	return nil
}

func (l *Link) UpdateOrCreate(v map[string]interface{}) error {
	if err := conn.MysqlConn.Model(l).Where("user_id=?", l.UserId).FirstOrCreate(l).UpdateColumns(v).Error; err != nil {
		return err
	}
	return nil
}

func (l *Link) GetLinkStatus() string {
	return map[int]string{
		VerifyInt:    "审核中",
		VerifyPass:   "审核通过",
		VerifyFailed: "审核不通过",
	}[l.VerifyStatus]
}
