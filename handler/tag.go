package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
)

type AdminTagListRequest struct {
	PageNum  int    `form:"pageNum"`
	PageSize int    `form:"pageSize"`
	Title    string `form:"title"`
}

func AdminTagList(c *gin.Context) {
	req := AdminTagListRequest{}
	resp := pb.ListByClassResp{}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	req.PageSize, req.PageNum = normalizePagination(req.PageSize, req.PageNum)
	tagResp, total, err := service.GetTagList(
		req.PageSize,
		req.PageNum,
		req.Title,
		c.Query("params[beginTime]"),
		c.Query("params[endTime]"),
	)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	tagResp.Msg = strconv.Itoa(int(total))
	c.ProtoBuf(http.StatusOK, tagResp)
}

func AdminAddTag(c *gin.Context) {
	req := pb.AdminEditCategoryRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.AddTag(req.Title, req.Description); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminEditTag(c *gin.Context) {
	req := pb.AdminEditCategoryRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.UpdateTag(int(req.Id), req.Title, req.Description); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminDeleteTag(c *gin.Context) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.DeleteTags(ids); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
