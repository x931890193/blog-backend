package entity

import "blog-backend/model/conn"

func (c *Category) AddOneCategory() (*Category, error) {
	err := conn.MysqlConn.Model(&c).Create(c).Error
	if err != nil {
		return nil, err
	}
	return c, nil
}

func (c *Category) GetAllCategory() ([]*Category, error) {
	var res []*Category
	if err := conn.MysqlConn.Model(&Category{}).Find(&res).Error; err != nil {
		return nil, err
	}
	return res, nil
}
