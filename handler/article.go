package handler

import (
	"blog-backend/logger"
	pb "blog-backend/proto"
	"blog-backend/service"
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ArticleListRequest struct {
	Like        uint `form:"like"`
	Collect     uint `form:"collect"`
	PageSize    uint `form:"pageSize"`
	CurrentPage uint `form:"currentPage"`
}

func GetArticleList(c *gin.Context) {
	req := ArticleListRequest{}
	resp := pb.ArticleListResp{}

	if err := c.ShouldBindQuery(&req); err != nil {
		logger.Logger.Error(fmt.Sprintf("parse ArticleListRequest error: ", c.Request.URL))
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
	}
	articles, err := service.ArticleList(int(req.PageSize), int(req.CurrentPage))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
	}
	resp.List = articles
	resp.Pagination = &pb.Pagination{
		CountTotal: 10,
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
