package service

import (
	"blog-backend/model/conn"
	"blog-backend/model/entity"
	"gorm.io/gorm"
)

func UpdateLikeOrCollect(userId int, articleId int, likeOrCollect, flag bool) error {
	articleObj := entity.Article{BaseModel: entity.BaseModel{ID: articleId}}
	like := entity.Like{ArticleId: uint(articleId), UserId: uint(userId)}
	collect := entity.Collection{ArticleId: uint(articleId), UserId: uint(userId)}

	err := conn.MysqlConn.Transaction(func(tx *gorm.DB) error {
		article, err := articleObj.GetOne()
		if err != nil {
			return err
		}
		if likeOrCollect {
			oneLike, err := like.GetOrCreate()
			if err != nil {
				return err
			}
			if flag {
				oneLike.BaseModel.IsDelete = false
				article.LikeCount += 1
			} else {
				oneLike.BaseModel.IsDelete = true
				article.LikeCount -= 1
			}
			err = tx.Save(&oneLike).Error
			if err != nil {
				return err
			}

		} else {
			oneCollect, err := collect.GetOrCreate()
			if err != nil {
				return err
			}
			if flag {
				oneCollect.BaseModel.IsDelete = false
				article.CollectCount += 1
			} else {
				oneCollect.BaseModel.IsDelete = true
				article.CollectCount -= 1
			}
			if err := tx.Save(&oneCollect).Error; err != nil {
				return err
			}
		}
		if err := tx.Updates(&article).Error; err != nil {
			return err
		}
		return nil
	})
	if err != nil {
		return err
	}
	return nil
}

func GetLikeAndCollect(userId int, articleId int) (isLike, isCollect bool, err error) {
	like := entity.Like{ArticleId: uint(articleId), UserId: uint(userId)}
	one, err := like.GetOne()
	if err != nil {
		return false, false, err
	}
	if one != nil {
		isLike = !one.IsDelete
	}
	collect := entity.Collection{ArticleId: uint(articleId), UserId: uint(userId)}
	collection, err := collect.GetOne()
	if err != nil {
		isCollect = false
	}
	if collection != nil {
		isCollect = !collection.IsDelete
	}
	return isLike, isCollect, nil
}
