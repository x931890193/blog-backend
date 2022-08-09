package handler

import (
	"blog-backend/cache"
	pb "blog-backend/proto"
	"blog-backend/service"
	"blog-backend/utils/captcha"
	"github.com/gin-gonic/gin"
	"net/http"
)

func GenerateAdmin(c *gin.Context) {

}

func GetCaptcha(c *gin.Context) {
	id, base64img, err := captcha.Generate()
	if err != nil {

	}
	c.JSON(http.StatusOK, gin.H{
		"id":  id,
		"img": base64img,
	})
}

func AdminLogin(c *gin.Context) {
	//res := proto.BaseResponse{}
	requestData := pb.LoginAdminRequest{}
	if err := c.Bind(&requestData); err != nil {
		c.ProtoBuf(http.StatusBadRequest, pb.LoginAdminResp{
			Code: uint32(ParamsError),
			Msg:  ConvertMsg(ParamsError, err.Error()),
		})
		return
	}
	resp := pb.LoginAdminResp{}
	token, err := service.UserAuth(requestData.Username, requestData.Password)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "用户名或密码错误")
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
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
	} else {
		resp.Avatar = userInfo.Avatar
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func LoginOut(c *gin.Context) {
	token := c.GetHeader("token")
	cache.Client.Del(token)
}
