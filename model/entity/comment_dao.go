package entity

import (
	"blog-backend/model/conn"
	"errors"
	"gorm.io/gorm"
	"strings"
)

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
	if pageSize <= 0 {
		pageSize = 10
	}
	if CurrentPage <= 0 {
		CurrentPage = 1
	}
	limitStart := (CurrentPage - 1) * pageSize
	if err := conn.MysqlConn.Where("article_id =? and parent_id = 0 and display = ?", c.ArticleId, true).Limit(pageSize).Offset(limitStart).Order("created_at DESC").Find(&parentRes).Error; err != nil {
		return nil, err
	}
	for _, parent := range parentRes {
		parentIds = append(parentIds, parent.ID)
	}
	if len(parentIds) == 0 {
		return parentRes, nil
	}

	if err := conn.MysqlConn.Where("article_id =? and parent_id in (?) and id not in (?) and display = ?", c.ArticleId, parentIds, parentIds, true).Find(&subRes).Error; err != nil {
		return nil, err
	}
	for _, parent := range subRes {
		subIds = append(subIds, parent.ID)

	}
	if len(subIds) > 0 {
		if err := conn.MysqlConn.Where("article_id =? and parent_id in (?) and id not in (?) and display = ?", c.ArticleId, subIds, subIds, true).Find(&grandRes).Error; err != nil {
			return nil, err
		}
	}
	parentRes = append(parentRes, subRes...)
	parentRes = append(parentRes, grandRes...)
	return parentRes, nil
}

func (c *Comment) CountRootComments() (int64, error) {
	var count int64
	if err := conn.MysqlConn.Model(c).Where("article_id =? and parent_id = 0 and display = ?", c.ArticleId, true).Count(&count).Error; err != nil {
		return 0, err
	}
	return count, nil
}

func (c *Comment) GetTopComment(limit int) ([]*Comment, error) {
	var res []*Comment
	if err := conn.MysqlConn.Model(&c).Where("parent_id = 0").Order("created_at DESC").Limit(limit).Find(&res).Error; err != nil {
		return nil, err
	}
	return res, nil
}

func (c *Comment) GetListByQuery(v map[string]interface{}) ([]*Comment, error) {
	comments := []*Comment{}
	if err := conn.MysqlConn.Model(c).Where(v).Find(&comments).Error; err != nil {
		return nil, err
	}
	return comments, nil
}

func GetAdminCommentList(pageSize, currentPage int, nickName, location, content, beginTime, endTime string) ([]*Comment, int64, error) {
	var comments []*Comment
	var total int64
	if pageSize <= 0 {
		pageSize = 10
	}
	if currentPage <= 0 {
		currentPage = 1
	}
	offset := (currentPage - 1) * pageSize
	query := conn.MysqlConn.Model(&Comment{})
	if v := strings.TrimSpace(nickName); v != "" {
		userSubQuery := conn.MysqlConn.Model(&User{}).Select("id").Where("user_name LIKE ?", "%"+v+"%")
		query = query.Where("user_id IN (?)", userSubQuery)
	}
	if v := strings.TrimSpace(location); v != "" {
		query = query.Where("location LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(content); v != "" {
		query = query.Where("content LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(beginTime); v != "" {
		query = query.Where("created_at >= ?", v+" 00:00:00")
	}
	if v := strings.TrimSpace(endTime); v != "" {
		query = query.Where("created_at <= ?", v+" 23:59:59")
	}
	if err := query.Count(&total).Error; err != nil {
		return nil, 0, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(offset).Find(&comments).Error; err != nil {
		return nil, 0, err
	}
	return comments, total, nil
}

func DeleteCommentsByIds(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&Comment{}, ids).Error
}

func UpdateCommentDisplay(id int, display bool) error {
	return conn.MysqlConn.Model(&Comment{}).Where("id = ?", id).Update("display", display).Error
}

func IncrementCommentVote(id int, field string) (*Comment, error) {
	if field != "good" && field != "bad" {
		return nil, errors.New("invalid comment vote field")
	}
	comment := &Comment{}
	err := conn.MysqlConn.Transaction(func(tx *gorm.DB) error {
		if err := tx.First(comment, id).Error; err != nil {
			return err
		}
		if err := tx.Model(comment).UpdateColumn(field, gorm.Expr(field+" + ?", 1)).Error; err != nil {
			return err
		}
		return tx.First(comment, id).Error
	})
	if err != nil {
		return nil, err
	}
	return comment, nil
}
