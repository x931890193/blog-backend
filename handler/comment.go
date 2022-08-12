package handler

import (
	"blog-backend/logger"
	pb "blog-backend/proto"
	"blog-backend/service"
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
)

func AddComment(c *gin.Context) {
	req := pb.CommentAddRequest{}
	resp := pb.CommentAddResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	token := c.Request.Header.Get("Token")
	user, err := service.AdminInfo(token)
	if err != nil {
		logger.Logger.Info("user not login!")
		user = service.NewTempUser()
	}
	data, err := service.AddComment(&req, user, c)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Data = data
	c.ProtoBuf(http.StatusOK, &resp)
}

func GetTopComments(c *gin.Context) {
	resp := &pb.TopCommentResp{}
	resp, err := service.GetTopComment()
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, resp)
}

// CommentListRequest query 参数
type CommentListRequest struct {
	PageSize    uint   `form:"pageSize"`
	CurrentPage uint   `form:"currentPage"`
	ArticleId   string `form:"articleId"`
	State       uint   `form:"state"`
}

func GetCommentList(c *gin.Context) {
	req := CommentListRequest{}
	resp := pb.CommentListResp{}

	if err := c.ShouldBindQuery(&req); err != nil {
		logger.Logger.Error(fmt.Sprintf("parse CommentListRequest error: ", c.Request.URL))
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	comments, err := service.GetCommentList(req.ArticleId, int(req.PageSize), int(req.CurrentPage))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
	}
	resp.List = comments.List
	resp.Pagination = comments.Pagination
	c.ProtoBuf(http.StatusOK, &resp)
}
