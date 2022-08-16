package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
	"strings"
)

func AddLink(c *gin.Context) {
	req := pb.LinkRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	err := service.AddLink(&req)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func LinkList(c *gin.Context) {
	var resp *pb.LinkListResp
	isAdmin := strings.Contains(c.Request.URL.String(), "admin")
	resp, err := service.LinkList(isAdmin)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	c.ProtoBuf(http.StatusOK, resp)
}
