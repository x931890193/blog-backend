package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
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

func EditLink(c *gin.Context) {
	req := pb.LinkRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if req.Id == 0 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "缺少友链ID")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	err := service.UpdateAdminLink(&req)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func DeleteLink(c *gin.Context) {
	resp := pb.BaseResp{}
	ids, err := parseIDs(c.Param("ids"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.DeleteLinks(ids); err != nil {
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
	pageNum, _ := strconv.Atoi(c.Query("pageNum"))
	if pageNum == 0 {
		pageNum, _ = strconv.Atoi(c.Query("currentPage"))
	}
	pageSize, _ := strconv.Atoi(c.Query("pageSize"))
	resp, err := service.LinkList(
		isAdmin,
		pageSize,
		pageNum,
		c.Query("title"),
		c.Query("description"),
		c.Query("params[beginTime]"),
		c.Query("params[endTime]"),
	)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, resp)
		return
	}
	c.ProtoBuf(http.StatusOK, resp)
}

func ChangeLinkDisplay(c *gin.Context) {
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
	if err := service.ChangeLinkDisplay(id, display); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminChangeLinkVerifyStatus(c *gin.Context) {
	resp := pb.BaseResp{}
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	pass, err := strconv.ParseBool(c.Param("pass"))
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := service.ChangeLinkVerifyStatus(id, pass); err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
