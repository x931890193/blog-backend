package handler

import (
	"blog-backend/logger"
	pb "blog-backend/proto"
	"blog-backend/service"
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
)

type ArticleListRequest struct {
	Like        uint `form:"like"`
	Collect     uint `form:"collect"`
	PageSize    int  `form:"pageSize"`
	CurrentPage int  `form:"currentPage"`
}

func GetArticleList(c *gin.Context) {
	req := ArticleListRequest{}
	resp := pb.ArticleListResp{}

	if err := c.ShouldBindQuery(&req); err != nil {
		logger.Logger.Error(fmt.Sprintf("parse ArticleListRequest error: ", c.Request.URL))
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	articles, err := service.ArticleList(req.PageSize, req.CurrentPage)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
	}
	resp.List = articles
	resp.Pagination = &pb.Pagination{
		CountTotal:  uint32(len(articles)),
		TotalPage:   10,
		CurrentPage: uint32(req.PageSize + 1),
		PageSize:    uint32(req.PageSize),
	}
	c.ProtoBuf(http.StatusOK, &resp)
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
	resp, err = service.GetOneAndUpdateClick(atoi)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	c.ProtoBuf(http.StatusOK, resp)
}

type AdminArticleListRequest struct {
	PageNum       int    `form:"pageNum"`
	pageSize      int    `form:"pageSize"`
	orderByColumn string `form:"orderByColumn"`
	isAsc         string `form:"isAsc"`
	params        interface{}
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
	list, err := service.ArticleListOrigin(req.pageSize, req.PageNum)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Total = uint32(len(list))
	for _, l := range list {
		resp.Rows = append(resp.Rows, &pb.AdminArticleListRespAdminArticleListBase{
			Title:      l.Title,
			Summary:    l.Summary,
			Comment:    l.CanComment,
			Weight:     uint32(l.Weight),
			Support:    l.Support,
			CreateTime: l.CreatedAt.Format("2006-01-02 15:04:05"),
			Id:         uint32(l.ID),
			Status:     !l.IsDelete,
			Category:   &pb.AdminArticleListRespAdminArticleListCategory{},
		})
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
