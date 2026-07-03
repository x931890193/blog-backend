package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
	"strings"
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
	resp.Author = siteInfo.Author
	resp.Beian = siteInfo.RecordNumber
	resp.Github = siteInfo.Git
	resp.Title = siteInfo.Title
	resp.Descriptions = siteInfo.Description
	resp.Keywords = siteInfo.Keywords
	resp.AlipayImage = siteInfo.AliPayImage
	resp.WechatPayImage = siteInfo.WeChatPayImage
	resp.Job = siteInfo.Job
	resp.LoveCount = uint32(service.SiteLoveCount(siteInfo))
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
	resp.LikeNum = service.SiteLoveCountString(siteInfo)
	resp.Descriptions = siteInfo.SelfDescription
	if strings.TrimSpace(resp.Descriptions) == "" {
		resp.Descriptions = service.DefaultSelfDescription()
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func LoveSite(c *gin.Context) {
	resp := pb.BaseResp{}
	if c.Query("dryRun") == "1" {
		siteInfo, err := service.GetSiteInfo()
		if err != nil {
			resp.Code = uint32(DbError)
			resp.Msg = ConvertMsg(DbError, err.Error())
			c.ProtoBuf(http.StatusOK, &resp)
			return
		}
		resp.Msg = service.SiteLoveCountString(siteInfo)
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	loveCount, err := service.IncreaseSiteLoveCount()
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Msg = strconv.Itoa(loveCount)
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
	c.ProtoBuf(http.StatusOK, &resp)
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
	resp.SelfDescriptions = siteInfo.SelfDescription
	if strings.TrimSpace(resp.SelfDescriptions) == "" {
		resp.SelfDescriptions = service.DefaultSelfDescription()
	}
	resp.Descriptions = siteInfo.Description
	resp.Author = siteInfo.Author
	resp.Beian = siteInfo.RecordNumber
	resp.Github = siteInfo.Git
	resp.Title = siteInfo.Title
	resp.Keywords = siteInfo.Keywords
	resp.AlipayImage = siteInfo.AliPayImage
	resp.WechatPayImage = siteInfo.WeChatPayImage
	resp.Job = siteInfo.Job
	resp.LoveCount = uint32(service.SiteLoveCount(siteInfo))
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminSetSiteInfo(c *gin.Context) {
	req := pb.SiteInfoReq{}
	resp := pb.BaseResp{}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	_, err := service.UpdateOrCreate(int(req.Id), map[string]interface{}{
		"author":          req.Author,
		"record_number":   req.Beian,
		"title":           req.Title,
		"description":     req.Descriptions,
		"keywords":        req.Keywords,
		"git":             req.Github,
		"job":             req.Job,
		"alipay_image":    req.AlipayImage,
		"wechatpay_image": req.WechatPayImage,
		"love_count":      int(req.LoveCount),
	})
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminPanelGroupResp(c *gin.Context) {
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

func AdminLineChartData(c *gin.Context) {
	resp := pb.LineChartDataResp{}
	typeData := c.Param("type")
	res, err := service.GetLineChartData(typeData)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
	}
	data := res.(map[string]interface{})
	resp.AxisData, _ = data["AxisData"].([]string)
	resp.ExpectedData, _ = data["ExpectedData"].([]uint32)
	resp.ActualData, _ = data["ActualData"].([]uint32)
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminDashboardAccess(c *gin.Context) {
	resp := pb.DashboardAccessResp{}
	result, err := service.AdminDashboardAccessData(500)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}

func AdminDashboardSpiderData(c *gin.Context) {
	resp := pb.DashboardSpiderResp{}
	result, err := service.AdminDashboardSpiderData(500)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, result)
}
