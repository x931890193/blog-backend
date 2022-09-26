package entity

import "blog-backend/model/conn"

func (r *Request) GetListByQuery(v map[string]interface{}) ([]*Request, error) {
	requests := []*Request{}
	if err := conn.MysqlConn.Model(r).Where(v).Find(&requests).Error; err != nil {
		return nil, err
	}
	return requests, nil
}
