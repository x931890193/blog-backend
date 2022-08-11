package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"encoding/json"
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
			Title:            item.Title,
			BrowseCount:      uint32(item.ClickTimes),
			ClassId:          uint32(item.ID),
			CollectCount:     1,
			CommentCount:     1,
			Content:          item.Summary,
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

func ArticleListOrigin(pageSize, currentPage int) ([]*entity.Article, error) {
	article := entity.Article{}
	articles, err := article.GetArticleListOrderCreateTime(pageSize, currentPage)
	if err != nil {
		return nil, err
	}
	return articles, nil
}

func AddArticle(request *pb.AdminArticleAddRequest) error {
	tags, _ := json.Marshal(request.TagTitleList)
	article := entity.Article{
		CategoryId:    uint(request.CategoryId),
		Tags:          string(tags),
		UserId:        0,
		Title:         request.Title,
		Summary:       request.Summary,
		Content:       request.Content,
		ClickTimes:    0,
		CanComment:    request.Comment,
		Weight:        uint(request.Weight),
		Support:       request.Support,
		HeaderImgType: uint(request.HeaderImgType),
		HeaderImg:     request.HeaderImg,
	}
	err := article.CreateOne()
	if err != nil {
		return err
	}
	return nil
}

func UpdateArticle(request *pb.AdminArticleAddRequest, id int) error {
	tags, _ := json.Marshal(request.TagTitleList)
	article := entity.Article{
		BaseModel: entity.BaseModel{
			ID: id,
		},
		CategoryId:    uint(request.CategoryId),
		Tags:          string(tags),
		UserId:        0,
		Title:         request.Title,
		Summary:       request.Summary,
		Content:       request.Content,
		ClickTimes:    0,
		CanComment:    request.Comment,
		Weight:        uint(request.Weight),
		Support:       request.Support,
		HeaderImgType: uint(request.HeaderImgType),
		HeaderImg:     request.HeaderImg,
	}
	err := article.UpdateById()
	if err != nil {
		return err
	}
	return nil
}

func GetAdminOne(id int) (*pb.AdminArticleOneResp, error) {
	article := &entity.Article{BaseModel: entity.BaseModel{ID: id}}
	one, err := article.GetOne()
	if err != nil {
		return nil, err
	}
	tagTitleList := []string{}
	err = json.Unmarshal([]byte(one.Tags), &tagTitleList)
	if err != nil {
		return nil, err
	}
	return &pb.AdminArticleOneResp{
		Id:            uint32(one.ID),
		Title:         one.Title,
		Summary:       one.Summary,
		CategoryId:    uint32(one.CategoryId),
		Support:       one.Support,
		Comment:       one.CanComment,
		HeaderImgType: uint32(one.HeaderImgType),
		HeaderImg:     one.HeaderImg,
		Weight:        uint32(one.Weight),
		TagTitleList:  tagTitleList,
		Content:       one.Content,
	}, nil
}

func GetOne(id int) (*pb.ArticleOneResp, error) {
	a := &entity.Article{BaseModel: entity.BaseModel{ID: id}}
	one, err := a.GetOne()
	if err != nil {
		return nil, err
	}
	tagTitleList := []string{}
	err = json.Unmarshal([]byte(one.Tags), &tagTitleList)
	if err != nil {
		return nil, err
	}
	return &pb.ArticleOneResp{
		Obj: &pb.Article{
			Title:            one.Title,
			BrowseCount:      uint32(one.ClickTimes),
			ClassId:          uint32(one.ID),
			CollectCount:     1,
			CommentCount:     1,
			Content:          one.Content,
			CreateDate:       one.CreatedAt.Format("2006-01-02 15:04:05"),
			IsHot:            true,
			IsRecommend:      true,
			LastModifiedDate: one.UpdatedAt.Format("2006-01-02 15:04:05"),
			LikeCount:        1,
			XId:              uint32(one.ID),
			XV:               1,
		},
	}, nil
}
