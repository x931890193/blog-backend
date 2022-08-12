package entity

import (
	"blog-backend/model/conn"
)

func (c *Category) AddOneCategory() (*Category, error) {
	err := conn.MysqlConn.Model(&c).Create(c).Error
	if err != nil {
		return nil, err
	}
	return c, nil
}

func (c *Category) GetAllCategory(pageSize, currentPage int) ([]*Category, error) {
	var res []*Category
	//result := Result{}
	//rows, err := conn.MysqlConn.Raw("select SQL_CALC_FOUND_ROWS * from category LIMIT 0,10;SELECT FOUND_ROWS() as total;").Rows()
	//if err != nil {
	//	return nil, err
	//}
	//fmt.Println(rows)
	query := conn.MysqlConn.Model(&Category{})
	if pageSize != 0 && currentPage != 0 {
		limitStart := (currentPage - 1) * pageSize
		query = query.Limit(pageSize).Offset(limitStart)
	}
	if err := query.Find(&res).Error; err != nil {
		return nil, err
	}
	return res, nil
}

func (c *Category) UpdateById() error {
	if err := conn.MysqlConn.Model(&Category{}).Where("id=?", c.ID).Updates(&c).Error; err != nil {
		return err
	}
	return nil
}
