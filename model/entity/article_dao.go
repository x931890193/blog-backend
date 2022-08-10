package entity

import "blog-backend/model/conn"

func (a *Article) GetArticleById() (*Article, error) {
	article := Article{}
	err := conn.MysqlConn.Model(Article{}).Where("id=?", a.ID).First(&article).Error
	if err != nil {
		return nil, err
	}
	return &article, nil
}

func (a *Article) GetArticleListOrderClickTime(limit int) ([]*Article, error) {
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

func (a *Article) GetArticleListOrderCreateTime(pageSize, CurrentPage int) ([]*Article, error) {
	var res []*Article
	limitStart := (CurrentPage - 1) * pageSize
	err := conn.MysqlConn.Model(Article{}).Order("created_at Desc").Limit(pageSize).Offset(limitStart).Find(&res).Error
	if err != nil {
		return nil, err
	}
	return res, nil
}

func (a *Article) CreateOne() error {
	if err := conn.MysqlConn.Model(&a).Create(a).Error; err != nil {
		return err
	}
	return nil
}
