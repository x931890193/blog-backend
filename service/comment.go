package service

import (
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"blog-backend/utils/mail"
	"blog-backend/utils/useragent"
	"fmt"
	"github.com/gin-gonic/gin"
	"strconv"
	"strings"
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
		_, err := article.GetOneAndUpdate(false)
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
		go SendEmailWhenComment(&comment, one.ParentId, user, article.ID)

	}
	pbComment := pb.Comment{
		XId:        strconv.Itoa(int(one.ID)),
		Avatar:     user.Avatar,
		Username:   user.UserName,
		Label:      userLabel(user.Label),
		CreateDate: one.CreatedAt.Format("2006-01-02 15:04:05"),
		Content:    one.Content,
		Children:   []*pb.Comment{},
		Good:       uint32(one.Good),
		Bad:        uint32(one.Bad),
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
			label = userLabel(user.Label)
		} else {
			tUser := NewTempUser()
			userName = tUser.UserName
			label = userLabel(tUser.Label)
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
			Good:           uint32(item.Good),
			Bad:            uint32(item.Bad),
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
	total, err := comment.CountRootComments()
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
	if pageSize <= 0 {
		pageSize = 10
	}
	if CurrentPage <= 0 {
		CurrentPage = 1
	}
	totalPage := int(total) / pageSize
	if int(total)%pageSize != 0 {
		totalPage++
	}
	resp.Pagination = &pb.Pagination{
		CountTotal:  uint32(total),
		TotalPage:   uint32(totalPage),
		CurrentPage: uint32(CurrentPage),
		PageSize:    uint32(pageSize),
	}

	return &resp, nil
}

func GetAdminCommentList(pageSize, currentPage int) (*pb.CommentListResp, error) {
	dbRes, total, err := entity.GetAdminCommentList(pageSize, currentPage, "", "", "", "", "")
	if err != nil {
		return nil, err
	}
	commentUserIds := []int{}
	for _, item := range dbRes {
		commentUserIds = append(commentUserIds, item.UserId)
	}
	userMap, err := GetUsersMapByIds(commentUserIds)
	if err != nil {
		return nil, err
	}
	resp := &pb.CommentListResp{List: []*pb.Comment{}}
	for _, item := range dbRes {
		userName := "游客"
		avatar := ""
		label := ""
		if user, ok := userMap[item.UserId]; ok {
			userName = user.UserName
			avatar = user.Avatar
			label = userLabel(user.Label)
		}
		resp.List = append(resp.List, &pb.Comment{
			XId:        strconv.Itoa(item.ID),
			Avatar:     avatar,
			Username:   userName,
			Label:      label,
			CreateDate: item.CreatedAt.Format("2006-01-02 15:04:05"),
			Content:    item.Content,
			Ip:         item.Ip,
			Ua:         item.Ua,
			Location:   item.Location,
			Os:         item.OS,
			Display:    item.Display,
			Good:       uint32(item.Good),
			Bad:        uint32(item.Bad),
			ParentId:   uint32(item.ParentId),
			Url:        strconv.Itoa(item.ArticleId),
		})
	}
	if pageSize <= 0 {
		pageSize = 10
	}
	if currentPage <= 0 {
		currentPage = 1
	}
	totalPage := int(total) / pageSize
	if int(total)%pageSize != 0 {
		totalPage++
	}
	resp.Pagination = &pb.Pagination{
		CountTotal:  uint32(total),
		TotalPage:   uint32(totalPage),
		CurrentPage: uint32(currentPage),
		PageSize:    uint32(pageSize),
	}
	return resp, nil
}

func GetAdminCommentTableList(pageSize, currentPage int, nickName, location, content, beginTime, endTime string) (*pb.AdminTableListResp, error) {
	dbRes, total, err := entity.GetAdminCommentList(pageSize, currentPage, nickName, location, content, beginTime, endTime)
	if err != nil {
		return nil, err
	}
	commentUserIds := []int{}
	articleIDs := []int{}
	for _, item := range dbRes {
		commentUserIds = append(commentUserIds, item.UserId)
		if item.ArticleId > 0 {
			articleIDs = append(articleIDs, item.ArticleId)
		}
	}
	userMap, err := GetUsersMapByIds(commentUserIds)
	if err != nil {
		return nil, err
	}
	article := &entity.Article{}
	articleMap, err := article.GetArticleMapsByIds(articleIDs)
	if err != nil {
		return nil, err
	}
	resp := &pb.AdminTableListResp{Total: uint32(total), Rows: []*pb.AdminTableRow{}}
	for _, item := range dbRes {
		userName := "游客"
		avatar := ""
		articleTitle := "未知文章"
		if user, ok := userMap[item.UserId]; ok {
			userName = user.UserName
			avatar = user.Avatar
		}
		if article, ok := articleMap[item.ArticleId]; ok {
			articleTitle = article.Title
		} else if item.ArticleId < 0 {
			articleTitle = NewTempArticle(item.ArticleId).Title
		}
		resp.Rows = append(resp.Rows, &pb.AdminTableRow{
			Id:         uint32(item.ID),
			Title:      articleTitle,
			NickName:   userName,
			UserName:   userName,
			Avatar:     avatar,
			Ip:         item.Ip,
			Location:   item.Location,
			Browser:    item.Ua,
			Os:         item.OS,
			Display:    item.Display,
			Content:    item.Content,
			Good:       uint32(item.Good),
			Bad:        uint32(item.Bad),
			ParentId:   uint32(item.ParentId),
			ArticleId:  uint32(item.ArticleId),
			Url:        "/#/detail?id=" + strconv.Itoa(item.ArticleId),
			CreateTime: item.CreatedAt.Format("2006-01-02 15:04:05"),
		})
	}
	return resp, nil
}

func DeleteComments(ids []int) error {
	return entity.DeleteCommentsByIds(ids)
}

func UpdateCommentDisplay(id int, display bool) error {
	return entity.UpdateCommentDisplay(id, display)
}

func VoteComment(id int, voteType string) (*pb.Comment, error) {
	field := "good"
	if strings.TrimSpace(voteType) == "bad" {
		field = "bad"
	}
	comment, err := entity.IncrementCommentVote(id, field)
	if err != nil {
		return nil, err
	}
	return &pb.Comment{
		XId:  strconv.Itoa(comment.ID),
		Good: uint32(comment.Good),
		Bad:  uint32(comment.Bad),
	}, nil
}

func userLabel(index int) string {
	if index < 0 || index >= len(config.UserTags) {
		return ""
	}
	return config.UserTags[index]
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
		article        = &entity.Article{}
	)
	res := &pb.TopCommentResp{BrowseList: []*pb.BrowseList{}, TopCommentList: []*pb.TopCommentList{}}
	siteInfo, err := GetSiteInfo()
	if err == nil {
		res.LoveCount = SiteLoveCountString(siteInfo)
	}
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
		if item, ok := userMap[commentItem.UserId]; ok {
			user = &item
		} else {
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

func SendEmailWhenComment(comment *entity.Comment, originCommentId uint, me *entity.User, articleId int) {
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
		logger.Logger.Error(err.Error())
		return
	}

	for _, user := range users {
		url := fmt.Sprintf("%s/#/detail/%d", config.Host, articleId)
		if articleId < 0 {
			url = fmt.Sprintf("%s/#/%s", config.Host, config.ArticleIdmapReverse[articleId])
		}
		newUpdate := mail.Comment{
			Site:          config.Host,
			Username:      u.UserName,
			Who:           me.UserName,
			Time:          comment.CreatedAt.Format("2006-01-02 15:04:05"),
			OriginComment: originComment.Content,
			NewComment:    comment.Content,
			Url:           url,
		}
		mail.SendEmail([]string{user.Email}, comment.Content, newUpdate, nil)
	}

}
