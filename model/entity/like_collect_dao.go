package entity

import (
	"blog-backend/model/conn"
	"gorm.io/gorm"
)

func (l *Like) GetOne() (*Like, error) {
	var like *Like
	if err := conn.MysqlConn.Model(&l).Where("article_id=? and user_id=?", l.ArticleId, l.UserId).First(&like).Error; err != nil && err != gorm.ErrRecordNotFound {
		return nil, err
	}
	return like, nil
}

func (l *Like) GetOrCreate() (*Like, error) {
	if err := conn.MysqlConn.Model(&l).Where("article_id=? and user_id=?", l.ArticleId, l.UserId).FirstOrCreate(&l).Error; err != nil {
		return nil, err
	}
	return l, nil
}

func (c *Collection) GetOne() (*Collection, error) {
	var collect *Collection
	if err := conn.MysqlConn.Model(&c).Where("article_id=? and user_id=?", c.ArticleId, c.UserId).First(&collect).Error; err != nil && err != gorm.ErrRecordNotFound {
		return nil, err
	}
	return collect, nil
}

func (c *Collection) GetOrCreate() (*Collection, error) {
	if err := conn.MysqlConn.Model(&c).Where("article_id=? and user_id=?", c.ArticleId, c.UserId).FirstOrCreate(&c).Error; err != nil {
		return nil, err
	}
	return c, nil
}
