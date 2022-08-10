package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
)

func AddCategory(c *gin.Context) {
	req := pb.AdminCategoryAddRequest{}
	resp := pb.AdminCategoryAddResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
	}
	_, err := service.AddCategory(req.Title, req.Description, req.Support)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

type AdminCategoryListRequest struct {
	PageNum       int    `form:"pageNum"`
	pageSize      int    `form:"pageSize"`
	orderByColumn string `form:"orderByColumn"`
	isAsc         string `form:"isAsc"`
	params        interface{}
}

func AdminCategoryList(c *gin.Context) {
	res, err := service.GetCategoryList()
	if err != nil {
		return
	}
	c.ProtoBuf(http.StatusOK, res)

}
