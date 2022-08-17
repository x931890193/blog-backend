package handler

import (
	"blog-backend/cache"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"blog-backend/service"
	"blog-backend/utils/captcha"
	"blog-backend/utils/github"
	"github.com/gin-gonic/gin"
	"net/http"
)

func GenerateAdmin(c *gin.Context) {

}

func GetCaptcha(c *gin.Context) {
	id, base64img, err := captcha.Generate()
	resp := pb.CaptchaResp{}
	if err != nil {
		resp.Code = uint32(LogicError)
		resp.Msg = ConvertMsg(LogicError, "验证码生成错误～")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Id = id
	resp.Img = base64img
	c.ProtoBuf(http.StatusOK, &resp)

}

func AdminLogin(c *gin.Context) {
	requestData := pb.LoginAdminRequest{}
	resp := pb.LoginAdminResp{}

	if err := c.Bind(&requestData); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusBadRequest, &resp)
		return
	}
	verifyResult := captcha.Verify(requestData.Id, requestData.Code)
	if !verifyResult {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "验证码错误～")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	token, err := service.UserAuth(requestData.Username, requestData.Password)
	if err != nil {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
	} else {
		resp.Token = token
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func AdminInfo(c *gin.Context) {
	token, _ := c.Get("admin")
	s := token.(string)
	resp := pb.AdminInfoResp{}
	userInfo, err := service.ParseToken(s)
	if err != nil {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
	} else {
		resp.Avatar = userInfo.Avatar
		resp.Name = userInfo.UserName
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func Routers(c *gin.Context) {
	resp := pb.AdminRouterResp{}
	resp.Data = []*pb.Component{
		{
			Component: "Layout",
			Meta:      &pb.ComponentMeta{Title: "文章管理", Icon: "documentation"},
			Path:      "/article",
			Children: []*pb.Component{
				{
					Component: "blog/blog/index",
					Name:      "index",
					Path:      "",
					Hidden:    true,
					Meta: &pb.ComponentMeta{
						Title:   "",
						NoCache: false,
						Icon:    "documentation",
					},
				},
				{
					Component: "blog/blog/index",
					Name:      "AddBlog",
					Path:      "index",
					Meta: &pb.ComponentMeta{
						Title:      "文章管理",
						NoCache:    false,
						Icon:       "documentation",
						ActiveMenu: "/article/index",
					},
				},
				{
					Component: "blog/blog/add",
					Name:      "BlogAdd",
					Path:      "add",
					Hidden:    true,
				},
				{
					Component: "blog/blog/edit",
					Name:      "BlogEdit",
					Path:      "edit/*",
					Hidden:    true,
				},
				{
					Component: "blog/category/index",
					Name:      "categories",
					Path:      "categories",
					Meta: &pb.ComponentMeta{
						Title:      "分类管理",
						NoCache:    false,
						ActiveMenu: "/article/categories",
						Icon:       "component",
					},
				},
				{
					Component: "blog/comment/index",
					Name:      "Comments",
					Path:      "comments",
					Meta: &pb.ComponentMeta{
						Title:      "评论管理",
						NoCache:    false,
						ActiveMenu: "/article/comments",
						Icon:       "message",
					},
				},
				{
					Component: "blog/tag/index",
					Name:      "Tags",
					Path:      "tags",
					Meta: &pb.ComponentMeta{
						Title:      "标签管理",
						NoCache:    false,
						ActiveMenu: "/article/tags",
						Icon:       "code",
					},
				},
			},
		},
		{
			Component: "Layout",
			Meta:      &pb.ComponentMeta{Title: "日志管理", Icon: "log"},
			Path:      "/log",
			Children: []*pb.Component{
				{
					Component: "log/loginLog/index",
					Name:      "",
					Path:      "",
					Hidden:    true,
				},
				{
					Component: "log/loginLog/index",
					Name:      "loginLog",
					Path:      "loginLog",
					Meta: &pb.ComponentMeta{
						Title:      "登陆日志",
						NoCache:    false,
						Icon:       "logininfor",
						ActiveMenu: "/log/loginLog",
					},
				},
				{
					Component: "log/operateLog/index",
					Name:      "operateLog",
					Path:      "operateLog",
					Meta: &pb.ComponentMeta{
						Title:      "操作日志",
						NoCache:    false,
						Icon:       "form",
						ActiveMenu: "/log/operateLog",
					},
				},
				{
					Component: "log/quartzLog/index",
					Name:      "quartzLog",
					Path:      "quartzLog",
					Meta: &pb.ComponentMeta{
						Title:      "任务日志",
						NoCache:    false,
						Icon:       "guide",
						ActiveMenu: "/log/quartzLog",
					},
				},
				{
					Component: "log/RealTimeLog/index",
					Name:      "RealTimeLog",
					Path:      "RealTimeLog",
					Meta: &pb.ComponentMeta{
						Title:      "实时日志",
						NoCache:    false,
						Icon:       "online",
						ActiveMenu: "/log/realTimeLog",
					},
				},
				{
					Component: "log/visitLog/index",
					Name:      "visitLog",
					Path:      "visitLog",
					Meta: &pb.ComponentMeta{
						Title:      "访问日志",
						NoCache:    false,
						Icon:       "people",
						ActiveMenu: "/log/visitLog",
					},
				},
			},
		},
		{
			Component: "Layout",
			Meta:      &pb.ComponentMeta{Title: "系统监控", Icon: "monitor"},
			Path:      "/monitor",
			Children: []*pb.Component{
				{
					Component: "monitor/blacklist/index",
					Name:      "",
					Path:      "",
					Hidden:    true,
				},
				{
					Component: "monitor/blacklist/index",
					Name:      "blacklist",
					Path:      "blacklist",
					Meta: &pb.ComponentMeta{
						Title:      "黑名单管理",
						NoCache:    false,
						Icon:       "password",
						ActiveMenu: "/monitor/blacklist",
					},
				},
				{
					Component: "monitor/online/index",
					Name:      "online",
					Path:      "online",
					Meta: &pb.ComponentMeta{
						Title:      "在线用户",
						NoCache:    false,
						Icon:       "online",
						ActiveMenu: "/monitor/online",
					},
				},
				{
					Component: "monitor/redis/index",
					Name:      "redis",
					Path:      "redis",
					Meta: &pb.ComponentMeta{
						Title:      "redis状态",
						NoCache:    false,
						Icon:       "password",
						ActiveMenu: "/monitor/redis",
					},
				},
				{
					Component: "monitor/server/index",
					Name:      "server",
					Path:      "server",
					Meta: &pb.ComponentMeta{
						Title:      "server状态",
						NoCache:    false,
						Icon:       "password",
						ActiveMenu: "/monitor/server",
					},
				},
			},
		},
		{
			Component: "Layout",
			Meta:      &pb.ComponentMeta{Title: "网站管理", Icon: "system"},
			Path:      "/system",
			Children: []*pb.Component{
				{
					Component: "system/carousel/index",
					Name:      "",
					Path:      "",
					Hidden:    true,
				},
				{
					Component: "system/carousel/index",
					Name:      "carousel",
					Path:      "carousel",
					Meta: &pb.ComponentMeta{
						Title:      "轮播图",
						NoCache:    false,
						Icon:       "example",
						ActiveMenu: "/system/carousel",
					},
				},
				{
					Component: "system/link/index",
					Name:      "link",
					Path:      "link",
					Meta: &pb.ComponentMeta{
						Title:      "友链",
						NoCache:    false,
						Icon:       "people",
						ActiveMenu: "/system/link",
					},
				},
				{
					Component: "system/notice/index",
					Name:      "notice",
					Path:      "notice",
					Meta: &pb.ComponentMeta{
						Title:      "公告",
						NoCache:    false,
						Icon:       "online",
						ActiveMenu: "/system/notice",
					},
				},
				{
					Component: "system/role/index",
					Name:      "role",
					Path:      "role",
					Meta: &pb.ComponentMeta{
						Title:      "角色管理",
						NoCache:    false,
						Icon:       "user",
						ActiveMenu: "/system/role",
					},
				},
				{
					Component: "system/setting/index",
					Name:      "setting",
					Path:      "setting",
					Meta: &pb.ComponentMeta{
						Title:      "网站设置",
						NoCache:    false,
						Icon:       "system",
						ActiveMenu: "/system/setting",
					},
				},
				{
					Component: "system/user/index",
					Name:      "user",
					Path:      "user",
					Meta: &pb.ComponentMeta{
						Title:      "用户管理",
						NoCache:    false,
						Icon:       "user",
						ActiveMenu: "/system/user",
					},
				},
			},
		},
		{
			Component: "Layout",
			Meta:      &pb.ComponentMeta{Title: "系统工具", Icon: "tool"},
			Path:      "/tool",
			Children: []*pb.Component{
				{
					Component: "tool/quartz/index",
					Name:      "",
					Path:      "",
					Hidden:    true,
				},
				{
					Component: "tool/quartz/index",
					Name:      "quartz",
					Path:      "quartz",
					Meta: &pb.ComponentMeta{
						Title:      "quartz状态",
						NoCache:    false,
						ActiveMenu: "/tool/quartz",
					},
				},
				{
					Component: "tool/storage/index",
					Name:      "storage",
					Path:      "storage",
					Meta: &pb.ComponentMeta{
						Title:      "存储状态",
						NoCache:    false,
						ActiveMenu: "/tool/storage",
					},
				},
			},
		},
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func LoginOut(c *gin.Context) {
	tokenAdmin := c.GetHeader("admin")
	tokenUser := c.GetHeader("user")
	cache.Client.Del(tokenAdmin, tokenUser)
}

// GitHubOauth GitHub 回调
func GitHubOauth(c *gin.Context) {
	githubToken, err := github.GetAccessToken(c.Query("code"))
	if err != nil {
		c.Redirect(302, "/")
		return
	}
	githubUser, err := github.GetUserInfo(githubToken.AccessToken)
	if err != nil {
		c.Redirect(302, "/")
		return
	}
	user, err := service.GetOrCreateGitHubUser(githubUser)
	if err != nil {
		c.Redirect(302, "/")
		return
	}
	token, err := user.GenerateToken()
	if err != nil {
		return
	}
	c.SetSameSite(http.SameSiteNoneMode)
	c.SetCookie("blog-token", token, int(entity.Expire), "/", "127.0.0.1", false, false)
	c.Redirect(302, "http://localhost:8087/#/message")
}

func UserInfo(c *gin.Context) {
	token, _ := c.Get("user")
	s := token.(string)
	resp := pb.UserInfoResp{}
	userInfo, err := service.ParseToken(s)
	if err != nil {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.UserId = uint32(userInfo.BaseModel.ID)
	link, err := service.GetLink(resp.UserId)
	if err != nil {
		return
	}
	resp.Status = 3
	resp.ReceiveUpdate = userInfo.ReceiveUpdate
	if userInfo.IsAdmin {
		resp.Status = 1
	}
	resp.Username = userInfo.UserName
	resp.Label = uint32(userInfo.Label)
	resp.Avatar = userInfo.Avatar
	resp.VerifyStatus = link.GetLinkStatus()
	resp.State = link.ShowLink
	resp.Linkname = link.Title
	resp.LinkUrl = link.Url
	resp.LogoUrl = link.HeaderImg
	resp.LinkDesc = link.Description
	c.ProtoBuf(http.StatusOK, &resp)
}

func Edit(c *gin.Context) {
	req := pb.EditUserInfoRequest{}
	resp := pb.UserInfoResp{}
	if err := c.Bind(&req); err != nil {
		c.ProtoBuf(http.StatusOK, &pb.BaseResp{
			Code: uint32(ParamsError),
			Msg:  ConvertMsg(ParamsError, err.Error()),
		})
		return
	}
	user, err := service.UpdateUser(&req)
	if err != nil {
		c.ProtoBuf(http.StatusOK, &pb.BaseResp{
			Code: uint32(ParamsError),
			Msg:  ConvertMsg(ParamsError, err.Error()),
		})
		return
	}
	resp.UserId = req.UserId
	link, err := service.UpdateLink(&req)
	if err != nil {
		c.ProtoBuf(http.StatusOK, &pb.BaseResp{
			Code: uint32(ParamsError),
			Msg:  ConvertMsg(ParamsError, err.Error()),
		})
		return
	}
	resp.Status = 3
	resp.ReceiveUpdate = user.ReceiveUpdate
	if user.IsAdmin {
		resp.Status = 1
	}
	resp.Username = user.UserName
	resp.Label = uint32(user.Label)
	resp.Avatar = user.Avatar
	resp.VerifyStatus = link.GetLinkStatus()
	resp.LogoUrl = link.HeaderImg
	resp.State = link.ShowLink
	resp.Linkname = link.Title
	resp.LinkUrl = link.Url
	resp.LogoUrl = link.HeaderImg
	resp.LinkDesc = link.Description
	resp.Token, _ = user.GenerateToken()
	c.ProtoBuf(http.StatusOK, &resp)
}
