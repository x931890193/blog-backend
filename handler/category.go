package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
)

func AddCategory(c *gin.Context) {
	req := pb.AdminCategoryAddRequest{}
	resp := pb.AdminCategoryAddResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	_, err := service.AddCategory(req.Title, req.Description, req.Support)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

type AdminCategoryListRequest struct {
	PageNum       int    `form:"pageNum"`
	PageSize      int    `form:"pageSize"`
	OrderByColumn string `form:"orderByColumn"`
	IsAsc         string `form:"isAsc"`
	Params        interface{}
}

func CategoryList(c *gin.Context) {
	req := AdminCategoryListRequest{}
	resp := &pb.AdminCategoryListResp{}
	if err := c.BindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	resp, err := service.GetCategoryList(req.PageSize, req.PageNum)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	c.ProtoBuf(http.StatusOK, resp)
}

func GetArticleByClass(c *gin.Context) {
	classId := c.Query("classId")
	atoi, err := strconv.Atoi(classId)
	if err != nil {
		atoi = 0
	}
	resp, err := service.GetArticleWithClassAndTags(uint(atoi))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}

	c.ProtoBuf(http.StatusOK, resp)
}

func EditCategory(c *gin.Context) {
	req := pb.AdminEditCategoryRequest{}
	resp := pb.AdminEditCategoryResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	err := service.UpdateCategoryById(int(req.Id), &req)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
