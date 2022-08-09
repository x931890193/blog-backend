package entity

import "blog-backend/model/conn"

func (c *Comment) AddOneComment(user *User) (*Comment, error) {
	if err := conn.MysqlConn.Model(&c).Create(c).Error; err != nil {
		return nil, err
	}

	return c, nil
}

func (c *Comment) GetCommentList(pageSize, CurrentPage int) ([]*Comment, error) {
	var (
		parentRes []*Comment
		subRes    []*Comment
		grandRes  []*Comment
		parentIds []int
		subIds    []int
	)
	limitStart := (CurrentPage - 1) * pageSize
	if err := conn.MysqlConn.Where("article_id =? and parent_id = 0", c.ArticleId).Limit(pageSize).Offset(limitStart).Order("created_at DESC").Find(&parentRes).Error; err != nil {

	}
	for _, parent := range parentRes {
		parentIds = append(parentIds, parent.ID)
	}

	if err := conn.MysqlConn.Where("article_id =? and parent_id in (?) and id not in (?)", c.ArticleId, parentIds, parentIds).Find(&subRes).Error; err != nil {
		return nil, err
	}
	for _, parent := range subRes {
		subIds = append(subIds, parent.ID)

	}
	if err := conn.MysqlConn.Where("article_id =? and parent_id in (?) and id not in (?)", c.ArticleId, subIds, subIds).Find(&grandRes).Error; err != nil {
		return nil, err
	}
	parentRes = append(parentRes, subRes...)
	parentRes = append(parentRes, grandRes...)
	return parentRes, nil
}

func (c *Comment) GetTopComment(limit int) ([]*Comment, error) {
	var res []*Comment
	if err := conn.MysqlConn.Model(&c).Where("parent_id = 0").Order("created_at DESC").Limit(limit).Find(&res).Error; err != nil {
		return nil, err
	}
	return res, nil
}
