package handler

import (
	pb "blog-backend/proto"
	"github.com/gin-gonic/gin"
	"net/http"
)

const (
	visitLog   = "visitLog"
	taskLog    = "taskLog"
	operateLog = "operateLog"
	loginLog   = "loginLog"
)

type logRequest struct {
	pageNum       int    `form:"pageNum" binding:"required,gte=1,lte=1000"`
	pageSize      int    `form:"pageSize" binding:"required,gte=1,lte=1000"`
	orderByColumn string `form:"orderByColumn" binding:"required"`
	isAsc         string `form:"isAsc" binding:"required"`
}

func Log(c *gin.Context) {
	req := logRequest{}
	resp := pb.LogResp{}

	logType, _ := c.GetQuery("logType")
	if err := c.BindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	switch logType {
	case loginLog:

		c.ProtoBuf(http.StatusOK, &resp)
		return
	case operateLog:
		c.ProtoBuf(http.StatusOK, &resp)
		return
	default:
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
}
