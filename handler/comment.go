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

func AddComment(c *gin.Context) {
	req := pb.CommentAddRequest{}
	resp := pb.CommentAddResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	token, exist := c.Get("user")
	if !exist {
		token = ""
	}
	s, _ := token.(string)
	user, err := service.ParseToken(s)
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
		logger.Logger.Error(fmt.Sprintf("parse CommentListRequest error: %s", c.Request.URL.String()))
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	comments, err := service.GetCommentList(req.ArticleId, int(req.PageSize), int(req.CurrentPage))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.List = comments.List
	resp.Pagination = comments.Pagination
	c.ProtoBuf(http.StatusOK, &resp)
}

func VoteComment(c *gin.Context) {
	resp := pb.CommentAddResp{}
	id, err := strconv.Atoi(c.Query("id"))
	if err != nil || id <= 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "评论ID不正确")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	voteType := c.Query("type")
	data, err := service.VoteComment(id, voteType)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Data = data
	c.ProtoBuf(http.StatusOK, &resp)
}

type AdminCommentListRequest struct {
	PageNum  int    `form:"pageNum"`
	PageSize int    `form:"pageSize"`
	NickName string `form:"nickName"`
	Location string `form:"location"`
	Content  string `form:"content"`
}

func AdminCommentList(c *gin.Context) {
	req := AdminCommentListRequest{}
	resp := pb.AdminTableListResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	req.PageSize, req.PageNum = normalizePagination(req.PageSize, req.PageNum)
	comments, err := service.GetAdminCommentTableList(
		req.PageSize,
		req.PageNum,
		req.NickName,
		req.Location,
		req.Content,
		c.Query("params[beginTime]"),
		c.Query("params[endTime]"),
	)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, comments)
}

func AdminDeleteComment(c *gin.Context) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.DeleteComments(ids); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminChangeCommentDisplay(c *gin.Context) {
	resp := pb.BaseResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	display, err := strconv.ParseBool(c.Param("display"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.UpdateCommentDisplay(id, display); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
