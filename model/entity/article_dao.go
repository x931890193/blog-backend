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

func GetArticleList(limit int) ([]*Article, error) {
	var res []*Article
	err := conn.MysqlConn.Model(Article{}).Order("click_times Desc").Limit(limit).Find(&res).Error
	if err != nil {
		return nil, err
	}
	return res, nil
}

func GetArticleMap(limit int) (map[int]*Article, error) {
	var (
		res        []*Article
		articleMap map[int]*Article
	)
	err := conn.MysqlConn.Model(Article{}).Order("click_times Desc").Limit(limit).Find(&res).Error
	if err != nil {
		return nil, err
	}
	for _, article := range res {
		articleMap[article.ID] = article
	}
	return articleMap, nil
}
