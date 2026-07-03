package handler

import (
	"blog-backend/logger"
	pb "blog-backend/proto"
	"blog-backend/service"
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
	"strings"
)

type ArticleListRequest struct {
	Like        uint `form:"like"`
	Collect     uint `form:"collect"`
	PageSize    int  `form:"pageSize"`
	CurrentPage int  `form:"currentPage"`
}

func normalizePagination(pageSize, currentPage int) (int, int) {
	if pageSize <= 0 {
		pageSize = 10
	}
	if pageSize > 100 {
		pageSize = 100
	}
	if currentPage <= 0 {
		currentPage = 1
	}
	return pageSize, currentPage
}

func calcTotalPage(count, pageSize uint32) uint32 {
	if pageSize == 0 {
		return 0
	}
	return (count + pageSize - 1) / pageSize
}

func parseIDs(raw string) ([]int, error) {
	ids := []int{}
	for _, part := range strings.Split(raw, ",") {
		part = strings.TrimSpace(part)
		if part == "" {
			continue
		}
		id, err := strconv.Atoi(part)
		if err != nil {
			return nil, err
		}
		ids = append(ids, id)
	}
	return ids, nil
}

func getArticleListWithUserId(c *gin.Context, req ArticleListRequest, resp *pb.ArticleListResp, userId int) {
	req.PageSize, req.CurrentPage = normalizePagination(req.PageSize, req.CurrentPage)
	articles, count, err := service.ArticleListWithUser(req.PageSize, req.CurrentPage, userId, req.Like, req.Collect)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	resp.List = articles
	total := uint32(count)
	resp.Pagination = &pb.Pagination{
		CountTotal:  total,
		TotalPage:   calcTotalPage(total, uint32(req.PageSize)),
		CurrentPage: uint32(req.CurrentPage),
		PageSize:    uint32(req.PageSize),
	}
	c.ProtoBuf(http.StatusOK, resp)
}

func getArticleList(c *gin.Context, req ArticleListRequest, resp *pb.ArticleListResp) {
	req.PageSize, req.CurrentPage = normalizePagination(req.PageSize, req.CurrentPage)
	articles, err := service.ArticleList(req.PageSize, req.CurrentPage)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	total := service.GetArticleTotalCount()
	resp.List = articles
	resp.Pagination = &pb.Pagination{
		CountTotal:  total,
		TotalPage:   calcTotalPage(total, uint32(req.PageSize)),
		CurrentPage: uint32(req.CurrentPage),
		PageSize:    uint32(req.PageSize),
	}
	c.ProtoBuf(http.StatusOK, resp)
}

func GetArticleList(c *gin.Context) {
	req := ArticleListRequest{}
	resp := pb.ArticleListResp{}

	if err := c.ShouldBindQuery(&req); err != nil {
		logger.Logger.Error(fmt.Sprintf("parse ArticleListRequest error: %s", c.Request.URL.String()))
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	token, exist := c.Get("user")
	if exist {
		tokenStr := token.(string)
		userinfo, err := service.ParseToken(tokenStr)
		if err != nil {
			resp.Code = uint32(ParamsError)
			resp.Msg = ConvertMsg(ParamsError, err.Error())
			c.ProtoBuf(http.StatusOK, &resp)
			return
		}
		userId := userinfo.ID
		if req.Like == 1 {
			getArticleListWithUserId(c, req, &resp, userId)
			return
		} else if req.Collect == 1 {
			getArticleListWithUserId(c, req, &resp, userId)
			return
		}
		getArticleList(c, req, &resp)
		return

	} else {
		getArticleList(c, req, &resp)
		return
	}

}

func AdminAddArticle(c *gin.Context) {
	req := pb.AdminArticleAddRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	err := service.AddArticle(&req)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)

}

func AdminGetArticle(c *gin.Context) {
	id := c.Param("id")
	resp := &pb.AdminArticleOneResp{}
	atoi, err := strconv.Atoi(id)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	resp, err = service.GetAdminOne(atoi)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	c.ProtoBuf(http.StatusOK, resp)
	return
}

func GetArticle(c *gin.Context) {
	id := c.Query("id")
	resp := &pb.ArticleOneResp{}
	atoi, err := strconv.Atoi(id)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	articleRes, err := service.GetOneAndUpdateClick(atoi)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	c.ProtoBuf(http.StatusOK, articleRes)
}

type AdminArticleListRequest struct {
	PageNum       int    `form:"pageNum"`
	PageSize      int    `form:"pageSize"`
	OrderByColumn string `form:"orderByColumn"`
	IsAsc         string `form:"isAsc"`
	Title         string `form:"title"`
	Summary       string `form:"summary"`
	Status        string `form:"status"`
	Support       string `form:"support"`
}

func AdminArticleList(c *gin.Context) {
	req := AdminArticleListRequest{}
	resp := pb.AdminArticleListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	req.PageSize, req.PageNum = normalizePagination(req.PageSize, req.PageNum)
	list, total, err := service.AdminArticleList(
		req.PageSize,
		req.PageNum,
		req.Title,
		req.Summary,
		req.Status,
		req.Support,
		c.Query("params[beginTime]"),
		c.Query("params[endTime]"),
	)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Total = uint32(total)
	categories, _ := service.GetCategoryMap()
	for _, l := range list {
		category := &pb.AdminArticleListRespAdminArticleListCategory{}
		if item, ok := categories[int(l.CategoryId)]; ok {
			category.Title = item.Name
			category.Description = item.SeoDesc
		}
		resp.Rows = append(resp.Rows, &pb.AdminArticleListRespAdminArticleListBase{
			Title:      l.Title,
			Summary:    l.Summary,
			HeaderImg:  l.HeaderImg,
			Comment:    l.CanComment,
			Weight:     uint32(l.Weight),
			Support:    l.Support,
			CreateTime: l.CreatedAt.Format("2006-01-02 15:04:05"),
			Id:         uint32(l.ID),
			Status:     !l.IsDelete,
			Category:   category,
		})
	}

	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminDeleteArticle(c *gin.Context) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.DeleteArticles(ids); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

type AdminArticleSwitchRequest struct {
	Id      int  `json:"id" form:"id"`
	Support bool `json:"support" form:"support"`
	Comment bool `json:"comment" form:"comment"`
}

func AdminChangeArticleSupport(c *gin.Context) {
	req := AdminArticleSwitchRequest{}
	resp := pb.BaseResp{}
	_ = c.ShouldBindJSON(&req)
	if req.Id == 0 {
		req.Id, _ = strconv.Atoi(c.Query("id"))
	}
	if c.Query("support") != "" {
		req.Support, _ = strconv.ParseBool(c.Query("support"))
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "id不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.UpdateArticleSupport(req.Id, req.Support); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminChangeArticleComment(c *gin.Context) {
	req := AdminArticleSwitchRequest{}
	resp := pb.BaseResp{}
	_ = c.ShouldBindJSON(&req)
	if req.Id == 0 {
		req.Id, _ = strconv.Atoi(c.Query("id"))
	}
	if c.Query("comment") != "" {
		req.Comment, _ = strconv.ParseBool(c.Query("comment"))
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "id不能为空")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.UpdateArticleComment(req.Id, req.Comment); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminEditArticle(c *gin.Context) {
	id := c.Param("id")
	req := pb.AdminArticleAddRequest{}
	resp := pb.BaseResp{}
	atoi, err := strconv.Atoi(id)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err = c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	err = service.UpdateArticle(&req, atoi)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
