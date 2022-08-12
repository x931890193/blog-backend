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

func CategoryList(c *gin.Context) {
	res, err := service.GetCategoryList()
	if err != nil {
		return
	}
	c.ProtoBuf(http.StatusOK, res)

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
