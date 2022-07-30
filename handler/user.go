package handler

import (
	"blog-backend/cache"
	pb "blog-backend/proto"
	service "blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
)

func GenerateAdmin(c *gin.Context) {

}

func AdminLogin(c *gin.Context) {
	//res := proto.BaseResponse{}
	requestData := pb.LoginAdminRequest{}
	if err := c.Bind(&requestData); err != nil {
		c.JSON(http.StatusBadRequest, pb.LoginAdminResp{
			Msg: "参数错误！",
		})
		return
	}
	resp := pb.LoginAdminResp{}
	token, err := service.Auth(requestData.Username, requestData.Password)
	if err != nil {
		resp.Code = 1
		resp.Msg = err.Error()
	} else {
		resp.Token = token
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminInfo(c *gin.Context) {
	token := c.Request.Header.Get("Token")
	resp := pb.AdminInfoResp{}
	userInfo, err := service.AdminInfo(token)
	if err != nil {
		resp.Code = 1
		resp.Msg = err.Error()
	} else {
		resp.Avatar = userInfo.Avatar
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func LoginOut(c *gin.Context) {
	token := c.GetHeader("token")
	cache.Client.Del(token)
}
