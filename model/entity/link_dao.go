package entity

import "blog-backend/model/conn"

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
