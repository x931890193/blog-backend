package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"errors"
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
		if req.Id == 0 {
			err = errors.New("not Enough Params")
		}
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

func PanelGroupResp(c *gin.Context) {
	resp := pb.PanelGroupResp{}
	res, err := service.GetPanelGroup()
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	data := res.(map[string]int)
	resp.UserCount = uint32(data["userCount"])
	resp.VisitorCount = uint32(data["visitorCount"])
	resp.BlogCount = uint32(data["blogCount"])
	c.ProtoBuf(http.StatusOK, &resp)
}

func LineChartData(c *gin.Context) {
	resp := pb.LineChartDataResp{}
	typeData := c.Param("type")
	res, err := service.GetLineChartData(typeData)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
	}
	data := res.(map[string][]uint32)
	resp.ActualData = data["ActualData"]
	resp.ExpectedData = data["ExpectedData"]
	resp.AxisData = data["AxisData"]
	c.ProtoBuf(http.StatusOK, &resp)
}
