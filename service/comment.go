package service

import (
	"blog-backend/config"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"strconv"
)

func AddComment(request *pb.CommentAddRequest, user *entity.User) (data *pb.Comment, err error) {
	article := &entity.Article{}
	if id, ok := config.ArticleIdmap[request.ArticleId]; ok {
		article.ID = id
	} else {
		article, err = entity.GetArticleById(request.ArticleId)
		if err != nil {
			return nil, err
		}
	}

	parentId, err := strconv.Atoi(request.ParentId)
	if err != nil {
		parentId = 0
	}

	comment := entity.Comment{
		UserId:    user.ID,
		ArticleId: article.ID,
		Content:   request.Content,
		ParentId:  uint(parentId),
	}
	one, err := comment.AddOneComment(user)
	if err != nil {
		return nil, err
	}
	pbComment := pb.Comment{
		XId:        strconv.Itoa(int(one.ID)),
		Avatar:     user.Avatar,
		Username:   user.UserName,
		Label:      user.Label,
		CreateDate: one.CreatedAt.Format("2006-01-02 15:04:05"),
		Content:    one.Content,
		Children:   []*pb.Comment{},
	}
	return &pbComment, nil
}

func getAllChildrenComment(dbRes []*entity.Comment, parentId uint, userMap map[int]entity.User) []*pb.Comment {
	var tmp []*pb.Comment
	children := []*pb.Comment{}
	for _, item := range dbRes {
		if item.ParentId != parentId {
			children = []*pb.Comment{}
			continue
		}
		userName := ""
		parentUsername := ""
		label := ""
		if user, ok := userMap[item.UserId]; ok {
			userName = user.UserName
			label = user.Label
		} else {
			tUser := NewTempUser()
			userName = tUser.UserName
			label = tUser.Label
		}
		if user, ok := userMap[int(item.ParentId)]; ok {
			parentUsername = user.UserName
		} else {
			tUser := NewTempUser()
			parentUsername = tUser.UserName
		}
		children = append(children, getAllChildrenComment(dbRes, uint(item.ID), userMap)...)
		tmp = append(tmp, &pb.Comment{
			XId:            strconv.Itoa(item.ID),
			Username:       userName,
			Label:          label,
			CreateDate:     item.CreatedAt.Format("2006-01-02 15:04:05"),
			Content:        item.Content,
			Children:       children,
			ParentUsername: parentUsername,
		})
		children = []*pb.Comment{}
	}
	return tmp
}

func GetCommentList(ArticleId string, pageSize, CurrentPage int) (*pb.CommentListResp, error) {
	var (
		res            []*pb.Comment
		commentUserIds []int
	)
	article := &entity.Article{}
	if id, ok := config.ArticleIdmap[ArticleId]; ok {
		article.ID = id
	} else {
		_, err := entity.GetArticleById(ArticleId)
		if err != nil {
			return nil, err
		}
	}
	comment := entity.Comment{ArticleId: article.ID}
	dbRes, err := comment.GetCommentListById(pageSize, CurrentPage)
	if err != nil {
		return nil, err
	}
	for _, item := range dbRes {
		//commentMap[item.ID] = item
		commentUserIds = append(commentUserIds, item.UserId)
	}
	userMap, err := GetUsersMapByIds(commentUserIds)
	if err != nil {
		return nil, err
	}
	res = getAllChildrenComment(dbRes, 0, userMap)
	resp := pb.CommentListResp{}
	resp.List = res
	resp.Pagination = &pb.CommentListResp_Pagination{
		CountTotal:  uint32(len(res)),
		TotalPage:   uint32(len(res)),
		CurrentPage: uint32(CurrentPage),
	}

	return &resp, nil
}

func NewTempArticle(id int) *entity.Article {
	article := &entity.Article{}
	if title, ok := config.ArticleIdmapReverse[id]; ok {
		article.Title = title
	} else {
		article.Title = "未知title"
	}
	return article
}

// GetTopComment comment  and most click
func GetTopComment() (*pb.TopCommentResp, error) {
	var (
		comment        entity.Comment
		commentUserIds []int
	)
	res := &pb.TopCommentResp{BrowseList: []*pb.BrowseList{}, TopCommentList: []*pb.TopCommentList{}}
	articleMap, err := entity.GetArticleMap(10)
	if err != nil {
		return nil, err
	}
	topCommentList, err := comment.GetTopComment(10)
	if err != nil {
		return nil, err
	}
	for _, item := range topCommentList {
		commentUserIds = append(commentUserIds, item.UserId)
	}
	userMap, err := GetUsersMapByIds(commentUserIds)
	if err != nil {
		return nil, err
	}
	for _, commentItem := range topCommentList {
		user := &entity.User{}
		article := &entity.Article{}
		if _, ok := userMap[commentItem.UserId]; !ok {
			user = NewTempUser()
		}

		if _, ok := articleMap[commentItem.ArticleId]; !ok {
			article = NewTempArticle(commentItem.ArticleId)
		} else {
			res.BrowseList = append(res.BrowseList, &pb.BrowseList{
				ArticleId: strconv.Itoa(commentItem.ArticleId),
				Title:     article.Title,
				Count:     uint32(article.ClickTimes),
			})
		}
		res.TopCommentList = append(res.TopCommentList, &pb.TopCommentList{
			ArticleId: strconv.Itoa(commentItem.ArticleId),
			Avatar:    user.Avatar,
			Title:     article.Title,
			Username:  user.UserName,
			Content:   commentItem.Content,
		})

	}
	return res, nil
}
