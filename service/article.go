package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
)

func ArticleList(pageSize, currentPage int) ([]*pb.Article, error) {
	article := entity.Article{}
	articles, err := article.GetArticleListOrderCreateTime(pageSize, currentPage)
	if err != nil {
		return nil, err
	}
	var res []*pb.Article
	for _, item := range articles {
		res = append(res, &pb.Article{
			BrowseCount:      uint32(item.ClickTimes),
			ClassId:          1,
			CollectCount:     1,
			CommentCount:     1,
			Content:          item.Content,
			CreateDate:       item.CreatedAt.Format("2006-01-02 15:04:05"),
			IsHot:            true,
			IsRecommend:      true,
			LastModifiedDate: item.UpdatedAt.Format("2006-01-02 15:04:05"),
			LikeCount:        1,
			XId:              uint32(item.ID),
			XV:               1,
		})
	}
	return res, nil
}
