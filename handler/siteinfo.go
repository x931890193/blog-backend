package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
)

func GetSiteInfo(c *gin.Context) {
	resp := pb.SiteInfoResp{}
	siteInfo, err := service.GetInfo()
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
	}

	resp.Author = siteInfo.Author
	resp.Beian = siteInfo.Beian
	resp.Github = siteInfo.Github
	c.ProtoBuf(http.StatusOK, &resp)
}
