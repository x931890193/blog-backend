package service

import (
	"blog-backend/cache"
	"blog-backend/config"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"blog-backend/utils/mail"
	"encoding/json"
	"fmt"
	"sort"
	"sync"
	"time"
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
			CollectCount:     uint32(item.CollectCount),
			CommentCount:     uint32(item.CommentCount),
			Content:          item.Summary,
			CreateDate:       item.CreatedAt.Format("2006-01-02 15:04:05"),
			IsHot:            true,
			IsRecommend:      true,
			LastModifiedDate: item.UpdatedAt.Format("2006-01-02 15:04:05"),
			LikeCount:        uint32(item.LikeCount),
			XId:              uint32(item.ID),
			XV:               1,
		})
	}
	return res, nil
}

func ArticleListWithUser(pageSize, currentPage int, userId int, like, collect uint) ([]*pb.Article, int, error) {
	article := entity.Article{}
	likeEntity := entity.Like{UserId: uint(userId)}
	collectEntity := entity.Collection{UserId: uint(userId)}

	if like == 1 {
		likes, err := likeEntity.GetListByUserid()
		if err != nil {
			return nil, 0, err
		}
		ids := []uint{}
		for _, item := range likes {
			ids = append(ids, item.ArticleId)
		}
		return articleListWithUser(article, pageSize, currentPage, ids)
	} else if collect == 1 {
		collects, err := collectEntity.GetListByUserid()
		if err != nil {
			return nil, 0, err
		}
		ids := []uint{}
		for _, item := range collects {
			ids = append(ids, item.ArticleId)
		}
		return articleListWithUser(article, pageSize, currentPage, ids)
	}
	return nil, 0, nil
}

func articleListWithUser(article entity.Article, pageSize, currentPage int, ids []uint) ([]*pb.Article, int, error) {
	articles, err := article.GetArticleListByIdsWithPage(pageSize, currentPage, ids)
	if err != nil {
		return nil, 0, err
	}
	var res []*pb.Article
	for _, item := range articles {
		res = append(res, &pb.Article{
			Title:            item.Title,
			BrowseCount:      uint32(item.ClickTimes),
			ClassId:          uint32(item.ID),
			CollectCount:     uint32(item.CollectCount),
			CommentCount:     uint32(item.CommentCount),
			Content:          item.Summary,
			CreateDate:       item.CreatedAt.Format("2006-01-02 15:04:05"),
			IsHot:            true,
			IsRecommend:      true,
			LastModifiedDate: item.UpdatedAt.Format("2006-01-02 15:04:05"),
			LikeCount:        uint32(item.LikeCount),
			XId:              uint32(item.ID),
			XV:               1,
		})
	}
	return res, len(ids), nil
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
		UserId:        1,
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
	go SendEmailWhenArticle(&article)
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
			CollectCount:     uint32(one.CollectCount),
			CommentCount:     uint32(one.CommentCount),
			Content:          one.Content,
			CreateDate:       one.CreatedAt.Format("2006-01-02 15:04:05"),
			IsHot:            true,
			IsRecommend:      true,
			LastModifiedDate: one.UpdatedAt.Format("2006-01-02 15:04:05"),
			LikeCount:        uint32(one.LikeCount),
			XId:              uint32(one.ID),
			XV:               1,
		},
	}, nil
}

func GetOneAndUpdateClick(id int) (*pb.ArticleOneResp, error) {
	a := &entity.Article{BaseModel: entity.BaseModel{ID: id}}
	one, err := a.GetOneAndUpdate(true)
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
			CollectCount:     uint32(one.CollectCount),
			CommentCount:     uint32(one.CommentCount),
			Content:          one.Content,
			CreateDate:       one.CreatedAt.Format("2006-01-02 15:04:05"),
			IsHot:            true,
			IsRecommend:      true,
			LastModifiedDate: one.UpdatedAt.Format("2006-01-02 15:04:05"),
			LikeCount:        uint32(one.LikeCount),
			XId:              uint32(one.ID),
			XV:               1,
		},
	}, nil
}

func GetArticleWithClassAndTags(categoryId uint) (*pb.ListByClassResp, error) {
	article := entity.Article{}
	category := entity.Category{}
	res := &pb.ListByClassResp{}
	articleList := []*pb.ListByClassResp_ArticleList{}
	catgoryList := []*pb.ListByClassResp_ClassList{}
	catgoryMap := map[int]*pb.ListByClassResp_ClassList{}
	categories, err := category.GetAllCategory(0, 0)
	for _, c := range categories {
		catgoryMap[c.ID] = &pb.ListByClassResp_ClassList{
			XId:   uint32(c.ID),
			Name:  c.Name,
			Count: 0,
		}
	}
	articles, err := article.GetAllArticle(0)
	if err != nil {
		return nil, err
	}
	articleYearMap := map[int][]*pb.ListByClassResp_List{}
	for _, a := range articles {
		year := a.CreatedAt.Year()
		_, ok := articleYearMap[year]
		if !ok {
			articleYearMap[year] = []*pb.ListByClassResp_List{}
		}
		if item, exist := catgoryMap[int(a.CategoryId)]; exist {
			item.Count++
		}
		if categoryId != 0 && a.CategoryId != categoryId {
			continue
		}
		articleYearMap[year] = append(articleYearMap[year], &pb.ListByClassResp_List{
			Title:      a.Title,
			XId:        uint32(a.ID),
			CreateDate: a.CreatedAt.Format("2006-01-02 15:04:05"),
		})
	}
	for k, v := range articleYearMap {
		articleList = append(articleList, &pb.ListByClassResp_ArticleList{
			Year: uint32(k),
			List: v,
		})
	}
	for _, v := range catgoryMap {
		catgoryList = append(catgoryList, &pb.ListByClassResp_ClassList{
			XId:   v.XId,
			Name:  v.Name,
			Count: v.Count,
		})
	}
	res.ArticleList = articleList
	sort.Slice(catgoryList, func(i, j int) bool { return catgoryList[i].XId < catgoryList[j].XId })
	res.ClassList = catgoryList
	return res, nil
}

func SendEmailWhenArticle(article *entity.Article) {
	u := entity.User{}
	users, err := u.GetListByQuery(map[string]interface{}{
		"receive_update": true,
	})
	if err != nil {
		return
	}
	wg := &sync.WaitGroup{}
	for _, user := range users {
		newUpdate := mail.NewUpdate{
			Username: user.UserName,
			Site:     config.Host,
			Time:     article.CreatedAt.Format("2006-01-02 15:04:05"),
			Title:    article.Title,
			Summary:  article.Summary,
			Url:      fmt.Sprintf("%s/#/detail/%d", config.Host, article.ID),
		}
		wg.Add(1)
		mail.SendEmail([]string{user.Email}, article.Title, newUpdate, wg)
	}

}

func GetArticleTotalCount() uint32 {
	count, err := cache.Client.Get("ArticleTotalCount").Int()
	if err != nil {
		article := entity.Article{}
		total, err := article.GetTotal()
		if err != nil {
			return 0
		}
		cache.Client.Set("ArticleTotalCount", total, time.Duration(-1))
	}
	return uint32(count)
}
