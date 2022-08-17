package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
)

func GetSiteInfo(c *gin.Context) {
	resp := pb.SiteInfoResp{}
	siteInfo, err := service.GetSiteInfo()
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Author = siteInfo.Auth
	resp.Beian = siteInfo.RecordNumber
	resp.Github = siteInfo.Git
	c.ProtoBuf(http.StatusOK, &resp)
}

func CatchMe(c *gin.Context) {
	resp := pb.AboutResp{}
	siteInfo, err := service.GetSiteInfo()
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.CatchMe = &pb.CatchMe{
		Git: siteInfo.Git,
		Job: siteInfo.Job,
	}
	resp.Id = uint32(siteInfo.ID)
	resp.LikeNum = "99999"
	resp.Descriptions = siteInfo.SelfDescription
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminAddOrUpdateAbout(c *gin.Context) {
	req := pb.UpdateAboutRequest{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	_, err := service.UpdateOrCreate(int(req.Id), map[string]interface{}{
		"self_description":      req.Content,
		"self_description_html": req.HtmlContent,
	})
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
}

func AdminGetSiteInfo(c *gin.Context) {
	resp := pb.SiteInfoResp{}
	siteInfo, err := service.GetSiteInfo()
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Id = uint32(siteInfo.ID)
	resp.Descriptions = siteInfo.SelfDescription
	resp.Author = siteInfo.Auth
	resp.Beian = siteInfo.RecordNumber
	resp.Github = siteInfo.Git
	c.ProtoBuf(http.StatusOK, &resp)
}
