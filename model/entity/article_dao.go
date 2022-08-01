package entity

import "blog-backend/model/conn"

func GetArticleById(id string) (*Article, error) {
	article := Article{}
	err := conn.MysqlConn.Model(Article{}).Where("id=?", id).First(&article).Error
	if err != nil {
		return nil, err
	}
	return &article, nil
}
