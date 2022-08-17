package service

import (
	"blog-backend/config"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"blog-backend/utils/mail"
	"blog-backend/utils/useragent"
	"fmt"
	"github.com/gin-gonic/gin"
	"strconv"
)

func AddComment(request *pb.CommentAddRequest, user *entity.User, c *gin.Context) (data *pb.Comment, err error) {
	article := &entity.Article{}
	if id, ok := config.ArticleIdmap[request.ArticleId]; ok {
		article.ID = id
	} else {
		article.ID, _ = strconv.Atoi(request.ArticleId)
		article, err = article.GetArticleById()
		if err != nil {
			return nil, err
		}
	}

	parentId, err := strconv.Atoi(request.ParentId)
	if err != nil {
		parentId = 0
	}
	v, exist := c.Get("client")
	ua := useragent.UserAgent{}
	if exist {
		ua, _ = v.(useragent.UserAgent)
	}
	comment := entity.Comment{
		UserId:    user.ID,
		ArticleId: article.ID,
		Content:   request.Content,
		ParentId:  uint(parentId),
		Ip:        ua.Ip,
		Ua:        fmt.Sprintf("%v %v", ua.Client["name"], ua.Client["version"]),
		OS:        fmt.Sprintf("%v %v", ua.OS.Name, ua.OS.Version),
	}
	one, err := comment.AddOneComment(user)
	if err != nil {
		return nil, err
	}
	if one.ParentId != 0 {
		go SendEmailWhenComment(&comment, one.ParentId, user)

	}
	pbComment := pb.Comment{
		XId:        strconv.Itoa(int(one.ID)),
		Avatar:     user.Avatar,
		Username:   user.UserName,
		Label:      config.UserTags[user.Label],
		CreateDate: one.CreatedAt.Format("2006-01-02 15:04:05"),
		Content:    one.Content,
		Children:   []*pb.Comment{},
	}
	return &pbComment, nil
}

func getAllChildrenComment(dbRes []*entity.Comment, parentId uint, userMap map[int]entity.User, commentIdUserIdMap map[int]int) []*pb.Comment {
	var tmp []*pb.Comment
	children := []*pb.Comment{}
	for _, item := range dbRes {
		if item.ParentId != parentId {
			children = []*pb.Comment{}
			continue
		}
		userName := ""
		parentUsername := ""
		avatar := ""
		label := ""
		if user, ok := userMap[item.UserId]; ok {
			userName = user.UserName
			avatar = user.Avatar
			label = config.UserTags[user.Label]
		} else {
			tUser := NewTempUser()
			userName = tUser.UserName
			label = config.UserTags[tUser.Label]
		}

		if user, ok := userMap[commentIdUserIdMap[int(item.ParentId)]]; ok {
			parentUsername = user.UserName
		} else {
			tUser := NewTempUser()
			parentUsername = tUser.UserName
		}
		children = append(children, getAllChildrenComment(dbRes, uint(item.ID), userMap, commentIdUserIdMap)...)
		tmp = append(tmp, &pb.Comment{
			Avatar:         avatar,
			XId:            strconv.Itoa(item.ID),
			Username:       userName,
			Label:          label,
			CreateDate:     item.CreatedAt.Format("2006-01-02 15:04:05"),
			Content:        item.Content,
			Children:       children,
			ParentUsername: parentUsername,
			Ip:             item.Ip,
			Ua:             item.Ua,
			Os:             item.OS,
		})
		children = []*pb.Comment{}
	}
	return tmp
}

func GetCommentList(ArticleId string, pageSize, CurrentPage int) (*pb.CommentListResp, error) {
	var (
		res                []*pb.Comment
		commentUserIds     []int
		commentIdUserIdMap = map[int]int{}
	)
	article := &entity.Article{}
	if id, ok := config.ArticleIdmap[ArticleId]; ok {
		article.ID = id
	} else {
		article.ID, _ = strconv.Atoi(ArticleId)
		_, err := article.GetArticleById()
		if err != nil {
			return nil, err
		}
	}
	comment := entity.Comment{ArticleId: article.ID}
	dbRes, err := comment.GetCommentList(pageSize, CurrentPage)
	if err != nil {
		return nil, err
	}
	for _, item := range dbRes {
		commentIdUserIdMap[item.ID] = item.UserId
		commentUserIds = append(commentUserIds, item.UserId)
	}
	userMap, err := GetUsersMapByIds(commentUserIds)
	if err != nil {
		return nil, err
	}
	res = getAllChildrenComment(dbRes, 0, userMap, commentIdUserIdMap)
	resp := pb.CommentListResp{}
	resp.List = res
	resp.Pagination = &pb.Pagination{
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
		articleIDs     []int
		article        *entity.Article
	)
	res := &pb.TopCommentResp{BrowseList: []*pb.BrowseList{}, TopCommentList: []*pb.TopCommentList{}}
	topCommentList, err := comment.GetTopComment(10)
	if err != nil {
		return nil, err
	}
	for _, item := range topCommentList {
		commentUserIds = append(commentUserIds, item.UserId)
		articleIDs = append(articleIDs, item.ArticleId)
	}

	userMap, err := GetUsersMapByIds(commentUserIds)
	if err != nil {
		return nil, err
	}
	articleMap, err := article.GetArticleMapsByIds(articleIDs)
	if err != nil {
		return nil, err
	}
	topArticles, err := article.GetArticleListOrderClickTime(10)
	if err != nil {
		return nil, err
	}
	for _, commentItem := range topCommentList {
		user := &entity.User{}
		if _, ok := userMap[commentItem.UserId]; !ok {
			user = NewTempUser()
		}
		article, ok := articleMap[commentItem.ArticleId]
		if !ok {
			article = NewTempArticle(commentItem.ArticleId)
		}
		res.TopCommentList = append(res.TopCommentList, &pb.TopCommentList{
			ArticleId: strconv.Itoa(commentItem.ArticleId),
			Avatar:    user.Avatar,
			Title:     article.Title,
			Username:  user.UserName,
			Content:   commentItem.Content,
		})

	}
	for _, a := range topArticles {
		res.BrowseList = append(res.BrowseList, &pb.BrowseList{
			ArticleId: strconv.Itoa(a.ID),
			Title:     a.Title,
			Count:     uint32(a.ClickTimes),
		})
	}
	return res, nil
}

func SendEmailWhenComment(comment *entity.Comment, originCommentId uint, me *entity.User) {
	originComment := &entity.Comment{}
	queryComments, err := originComment.GetListByQuery(map[string]interface{}{
		"id": originCommentId,
	})
	if err != nil {
		return
	}
	if len(queryComments) > 0 {
		originComment = queryComments[0]
	} else {
		// TODO
		return
	}
	u := entity.User{}
	users, err := u.GetListByQuery(map[string]interface{}{
		"receive_update": true,
		"id":             originComment.UserId,
	})
	if err != nil {
		// TOOD
		return
	}
	for _, user := range users {
		newUpdate := mail.Comment{
			Site:          "",
			Username:      u.UserName,
			Who:           me.UserName,
			Time:          comment.CreatedAt.Format("2006-01-02 15:04:05"),
			OriginComment: originComment.Content,
			NewComment:    comment.Content,
			Url:           "", //
		}
		mail.SendEmail([]string{user.Email}, comment.Content, newUpdate, nil)
	}

}
