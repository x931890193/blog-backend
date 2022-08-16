package entity

import "blog-backend/model/conn"

func (r *Resource) GetByUuid() error {
	if err := conn.MysqlConn.Model(r).Where("uuid=?", r.Uuid).Find(r).Error; err != nil {
		return err
	}
	return nil
}

func (r *Resource) Save() error {
	if err := conn.MysqlConn.Model(r).Create(r).Error; err != nil {
		return err
	}
	return nil
}
