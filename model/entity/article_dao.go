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

func (a *Article) GetArticleByIds(ids []int) ([]*Article, error) {
	articles := []*Article{}
	if len(ids) == 0 {
		return articles, nil
	}
	err := conn.MysqlConn.Model(Article{}).Where("id in ?", ids).Find(&articles).Error
	if err != nil {
		return nil, err
	}
	return articles, nil
}

func (a *Article) GetArticleListByIdsWithPage(pageSize, CurrentPage int, ids []uint) ([]*Article, error) {
	var res []*Article
	if len(ids) == 0 {
		return res, nil
	}
	if pageSize <= 0 {
		pageSize = 10
	}
	if CurrentPage <= 0 {
		CurrentPage = 1
	}
	limitStart := (CurrentPage - 1) * pageSize
	err := conn.MysqlConn.Model(Article{}).Where("id in ?", ids).Order("created_at Desc").Limit(pageSize).Offset(limitStart).Find(&res).Error
	if err != nil {
		return nil, err
	}
	return res, nil
}

func (a *Article) GetArticleMapsByIds(ids []int) (map[int]*Article, error) {
	articles := []*Article{}
	res := map[int]*Article{}
	if len(ids) == 0 {
		return res, nil
	}
	err := conn.MysqlConn.Model(Article{}).Where("id in ?", ids).Find(&articles).Error
	if err != nil {
		return nil, err
	}
	for _, item := range articles {
		res[item.ID] = item
	}
	return res, nil
}

func (a *Article) GetAllArticle(categoryID uint) ([]*Article, error) {
	var res []*Article
	query := conn.MysqlConn.Model(Article{})
	if categoryID != 0 {
		query = query.Where("category_id=?", categoryID)
	}
	err := query.Find(&res).Error
	if err != nil {
		return nil, err
	}
	return res, nil

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

	var res []*Article
	articleMap := map[int]*Article{}

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
	if pageSize <= 0 {
		pageSize = 10
	}
	if CurrentPage <= 0 {
		CurrentPage = 1
	}
	limitStart := (CurrentPage - 1) * pageSize
	err := conn.MysqlConn.Model(Article{}).Order("created_at Desc").Limit(pageSize).Offset(limitStart).Find(&res).Error
	if err != nil {
		return nil, err
	}
	return res, nil
}

func GetAdminArticleList(pageSize, CurrentPage int, title, summary, status, support, beginTime, endTime string) ([]*Article, int64, error) {
	var res []*Article
	var total int64
	if pageSize <= 0 {
		pageSize = 10
	}
	if CurrentPage <= 0 {
		CurrentPage = 1
	}
	query := conn.MysqlConn.Model(Article{})
	if title != "" {
		query = query.Where("title LIKE ?", "%"+title+"%")
	}
	if summary != "" {
		query = query.Where("summary LIKE ?", "%"+summary+"%")
	}
	if status == "true" {
		query = query.Where("is_delete = ?", false)
	} else if status == "false" {
		query = query.Where("is_delete = ?", true)
	}
	if support == "true" {
		query = query.Where("support = ?", true)
	} else if support == "false" {
		query = query.Where("support = ?", false)
	}
	if beginTime != "" {
		query = query.Where("created_at >= ?", beginTime+" 00:00:00")
	}
	if endTime != "" {
		query = query.Where("created_at <= ?", endTime+" 23:59:59")
	}
	if err := query.Count(&total).Error; err != nil {
		return nil, 0, err
	}
	limitStart := (CurrentPage - 1) * pageSize
	if err := query.Order("created_at Desc").Limit(pageSize).Offset(limitStart).Find(&res).Error; err != nil {
		return nil, 0, err
	}
	return res, total, nil
}

func (a *Article) CreateOne() error {
	if err := conn.MysqlConn.Model(&a).Create(a).Error; err != nil {
		return err
	}
	return nil
}

func (a *Article) GetOne() (*Article, error) {
	var article Article
	if err := conn.MysqlConn.Model(&a).Where("id=?", a.ID).First(&article).Error; err != nil {
		return nil, err
	}
	return &article, nil
}

func (a *Article) GetOneAndUpdate(isClick bool) (*Article, error) {
	var article Article
	if err := conn.MysqlConn.Model(&a).Where("id=?", a.ID).First(&article).Error; err != nil {
		return nil, err
	}
	if isClick {
		article.ClickTimes++
	} else {
		article.CommentCount++
	}
	if err := conn.MysqlConn.Save(&article).Error; err != nil {
		return nil, err
	}
	return &article, nil
}

func (a *Article) UpdateById() error {
	if err := conn.MysqlConn.Model(&a).Where("id=?", a.ID).Updates(&a).Error; err != nil {
		return err
	}
	return nil
}

func (a *Article) UpdateColumnsById(values map[string]interface{}) error {
	if err := conn.MysqlConn.Model(&Article{}).Where("id=?", a.ID).UpdateColumns(values).Error; err != nil {
		return err
	}
	return nil
}

func DeleteArticlesByIds(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&Article{}, ids).Error
}

func (a *Article) GetListByQuery(v map[string]interface{}) ([]*Article, error) {
	articles := []*Article{}
	if err := conn.MysqlConn.Model(a).Where(v).Find(&articles).Error; err != nil {
		return nil, err
	}
	return articles, nil
}

func (a *Article) GetTotal() (int64, error) {
	var c int64
	if err := conn.MysqlConn.Model(a).Count(&c).Error; err != nil {
		return 0, err
	}
	return c, nil
}

func ArticleContentExists(marker string) (bool, error) {
	var count int64
	if err := conn.MysqlConn.Model(Article{}).Where("content LIKE ?", "%"+marker+"%").Count(&count).Error; err != nil {
		return false, err
	}
	return count > 0, nil
}
