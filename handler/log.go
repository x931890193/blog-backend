package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
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
	PageNum       int    `form:"pageNum"`
	PageSize      int    `form:"pageSize"`
	OrderByColumn string `form:"orderByColumn"`
	IsAsc         string `form:"isAsc"`
}

func Log(c *gin.Context) {
	req := logRequest{}
	resp := pb.LogResp{}

	logType := c.Param("LogType")
	if logType == "" {
		logType, _ = c.GetQuery("logType")
	}
	if err := c.ShouldBindQuery(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	result, err := service.AdminDashboardLogRows(logType, req.PageSize, req.PageNum)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}
