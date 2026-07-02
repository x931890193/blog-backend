package router

import (
	"blog-backend/handler"
	"blog-backend/middleware"
	"github.com/gin-gonic/gin"
	"os"
)

var runMode string

func init() {
	runMode = gin.DebugMode
	if os.Getenv("PROGRAM_ENV") == "prod" {
		runMode = gin.ReleaseMode
	}
}

func SetupServer() *gin.Engine {
	gin.SetMode(runMode)
	router := gin.Default()
	//router.Use(middleware.AuthMiddleware())
	//router.Use(middleware.LogMiddleware())
	router.HandleMethodNotAllowed = true
	router.GET("/", handler.Hello)
	router.Use(middleware.RequestMiddleware())
	router.Use(middleware.BaseAuthMiddleware())
	admin := router.Group("/admin")
	{
		admin.POST("/generate", handler.GenerateAdmin)
		admin.GET("/captcha", handler.GetCaptcha)
		admin.POST("/login", handler.AdminLogin)
		admin.Use(middleware.AuthMiddleware())
		admin.GET("/info", handler.AdminInfo)
		admin.GET("/routers", handler.Routers)
		admin.POST("/logout", handler.LoginOut)
		admin.GET("/article/list", handler.AdminArticleList)
		admin.POST("/article/add", handler.AdminAddArticle)
		admin.POST("/article/support", handler.AdminChangeArticleSupport)
		admin.POST("/article/comment", handler.AdminChangeArticleComment)
		admin.POST("/article/category/add", handler.AddCategory)
		admin.POST("/article/category/edit", handler.EditCategory)
		admin.GET("/article/category/list", handler.CategoryList)
		admin.DELETE("/article/category/:ids", handler.DeleteCategory)
		admin.GET("/article/:id", handler.AdminGetArticle)
		admin.POST("/article/:id", handler.AdminEditArticle)
		admin.DELETE("/article/:ids", handler.AdminDeleteArticle)
		admin.GET("/blog/comment/list", handler.AdminCommentList)
		admin.PUT("/blog/comment/:id/display/:display", handler.AdminChangeCommentDisplay)
		admin.POST("/blog/comment/:id/display/:display", handler.AdminChangeCommentDisplay)
		admin.DELETE("/blog/comment/:ids", handler.AdminDeleteComment)
		admin.GET("/blog/tag/list", handler.AdminTagList)
		admin.POST("/blog/tag/add", handler.AdminAddTag)
		admin.POST("/blog/tag/edit", handler.AdminEditTag)
		admin.DELETE("/blog/tag/:ids", handler.AdminDeleteTag)
		admin.POST("/link/add", handler.AddLink)
		admin.POST("/link/edit", handler.EditLink)
		admin.GET("/link/list", handler.LinkList)
		admin.DELETE("/link/:ids", handler.DeleteLink)
		admin.POST("/link/:id/display/:display", handler.ChangeLinkDisplay)
		admin.PUT("/system/link/pass/:id/:pass", handler.AdminChangeLinkVerifyStatus)
		admin.POST("/system/link/pass/:id/:pass", handler.AdminChangeLinkVerifyStatus)
		admin.GET("/about/get", handler.AdminGetSiteInfo)
		admin.POST("/about/edit", handler.AdminAddOrUpdateAbout)
		// dashboard
		admin.GET("/dashboard/panelGroup", handler.AdminPanelGroupResp)
		admin.GET("/dashboard/lineChartData/:type", handler.AdminLineChartData)
		admin.GET("/dashboard/access", handler.AdminDashboardAccess)
		admin.GET("/dashboard/spiderData", handler.AdminDashboardSpiderData)
		admin.GET("/dashboard/log/:LogType", handler.Log)

		// dashboard end
		admin.GET("/system/setting/siteSetting", handler.AdminGetSiteInfo)
		admin.POST("/system/setting/siteSetting", handler.AdminSetSiteInfo)
		admin.GET("/system/user/list", handler.AdminUserList)
		admin.GET("/system/user/profile", handler.AdminUserProfile)
		admin.PUT("/system/user/profile", handler.AdminUserProfileUpdate)
		admin.PUT("/system/user/profile/updatePwd", handler.AdminUserProfileUpdatePassword)
		admin.POST("/system/user/profile/avatar", handler.UploadFile)
		admin.GET("/system/user/:id", handler.AdminUserOne)
		admin.POST("/system/user", handler.AdminUserAdd)
		admin.PUT("/system/user", handler.AdminUserEdit)
		admin.DELETE("/system/user/:ids", handler.AdminUserDelete)
		admin.PUT("/system/user/resetPwd", handler.AdminUserResetPassword)
		admin.PUT("/system/user/changeStatus", handler.AdminUserChangeStatus)
		admin.GET("/system/role/list", handler.AdminRoleList)
		admin.PUT("/system/role/changeStatus", handler.AdminRoleChangeStatus)
		admin.GET("/system/role/:id", handler.AdminRoleOne)
		admin.POST("/system/role", handler.AdminRoleAdd)
		admin.PUT("/system/role", handler.AdminRoleUpdate)
		admin.DELETE("/system/role/:ids", handler.AdminRoleDelete)
		admin.GET("/system/dict/data/dictType/:dictType", handler.AdminDictData)
		admin.GET("/system/notice/list", handler.AdminNoticeList)
		admin.GET("/system/notice/:id", handler.AdminNoticeOne)
		admin.POST("/system/notice", handler.AdminNoticeAdd)
		admin.PUT("/system/notice", handler.AdminNoticeUpdate)
		admin.DELETE("/system/notice/:ids", handler.AdminNoticeDelete)
		admin.GET("/system/carousel/list", handler.AdminCarouselList)
		admin.GET("/system/carousel/:id", handler.AdminCarouselOne)
		admin.POST("/system/carousel", handler.AdminCarouselAdd)
		admin.PUT("/system/carousel", handler.AdminCarouselUpdate)
		admin.DELETE("/system/carousel/:ids", handler.AdminCarouselDelete)
		admin.POST("/system/carousel/:id/display/:display", handler.AdminCarouselDisplay)
		admin.POST("/system/carousel/:id/target/:target", handler.AdminCarouselTarget)
		admin.PUT("/system/carousel/:id/display/:display", handler.AdminCarouselDisplay)
		admin.PUT("/system/carousel/:id/target/:target", handler.AdminCarouselTarget)
		admin.GET("/monitor/blacklist/list", handler.AdminBlacklistList)
		admin.GET("/monitor/blacklist/:id", handler.AdminBlacklistOne)
		admin.POST("/monitor/blacklist", handler.AdminBlacklistAdd)
		admin.PUT("/monitor/blacklist", handler.AdminBlacklistUpdate)
		admin.DELETE("/monitor/blacklist/:ids", handler.AdminBlacklistDelete)
		admin.GET("/monitor/online/list", handler.AdminOnlineList)
		admin.DELETE("/monitor/online/:tokenId", handler.AdminOnlineDelete)
		admin.GET("/monitor/redis/list", handler.AdminRedisInfo)
		admin.GET("/monitor/server", handler.AdminServerInfo)
		admin.GET("/log/loginLog/list", handler.AdminRequestLoginLogList)
		admin.GET("/log/operateLog/list", handler.AdminRequestOperateLogList)
		admin.GET("/log/quartzLog/list", handler.AdminQuartzLogList)
		admin.GET("/log/quartzLog/:id", handler.AdminQuartzLogOne)
		admin.GET("/log/visitLog/list", handler.AdminRequestVisitLogList)
		admin.DELETE("/log/loginLog/clean", handler.AdminRequestLoginLogClean)
		admin.DELETE("/log/operateLog/clean", handler.AdminRequestOperateLogClean)
		admin.DELETE("/log/quartzLog/clean", handler.AdminQuartzLogClean)
		admin.DELETE("/log/visitLog/clean", handler.AdminRequestVisitLogClean)
		admin.DELETE("/log/loginLog/:ids", handler.AdminRequestLoginLogDelete)
		admin.DELETE("/log/operateLog/:ids", handler.AdminRequestOperateLogDelete)
		admin.DELETE("/log/quartzLog/:ids", handler.AdminQuartzLogDelete)
		admin.DELETE("/log/visitLog/:ids", handler.AdminRequestVisitLogDelete)
		admin.GET("/tool/quartz/list", handler.AdminQuartzJobList)
		admin.GET("/tool/quartz/:id", handler.AdminQuartzJobOne)
		admin.POST("/tool/quartz", handler.AdminQuartzJobAdd)
		admin.PUT("/tool/quartz", handler.AdminQuartzJobUpdate)
		admin.PUT("/tool/quartz/exe/:id", handler.AdminQuartzJobExecute)
		admin.PUT("/tool/quartz/status/:id", handler.AdminQuartzJobStatus)
		admin.DELETE("/tool/quartz/:ids", handler.AdminQuartzJobDelete)
		admin.GET("/tool/qiNiu/list", handler.AdminStorageList)
		admin.GET("/tool/localStorage/list", handler.AdminStorageList)
		admin.GET("/tool/qiNiu/download/:id", handler.AdminBaseOK)
		admin.POST("/tool/qiNiu/synchronize", handler.AdminBaseOK)
		admin.DELETE("/tool/qiNiu/:ids", handler.AdminStorageDelete)
		admin.DELETE("/tool/localStorage/:ids", handler.AdminStorageDelete)
		// tool
		admin.POST("/tool/qiNiu/upload", handler.UploadFile)

		// end tool

	}
	// article
	article := router.Group("/article")
	{
		article.GET("/list", handler.GetArticleList)
		article.GET("/category/list", handler.CategoryList)
		article.GET("/getListByClass", handler.GetArticleByClass) // 归档标签
		article.GET("/getInfo", handler.GetArticle)
	}
	// comment
	comment := router.Group("/comment")
	{
		comment.GET("/list", handler.GetCommentList)
		comment.GET("/info")
		comment.POST("/add", handler.AddComment)
		comment.GET("/getTopComment", handler.GetTopComments)
	}
	// resource
	resource := router.Group("/resource")
	{
		resource.GET("/site_info", handler.GetSiteInfo)
		resource.GET("/about", handler.CatchMe)
		resource.POST("/upload", handler.UploadFile)
	}
	// site love
	love := router.Group("/love")
	{
		love.POST("/add", handler.LoveSite)
	}
	// user
	user := router.Group("/user")
	{
		user.GET("/github/oauth", handler.GitHubOauth)
		user.POST("/logout", handler.LoginOut)
		user.GET("/getUserInfo", handler.UserInfo)
		user.POST("/edit", handler.Edit)
	}
	// reward
	reward := router.Group("/reward")
	{
		reward.GET("/list", handler.RewardList)
	}
	// links
	links := router.Group("/link")
	{
		links.GET("/list", handler.LinkList)
	}
	// like collect
	like := router.Group("/likeOrCollect")
	{

		like.POST("/edit", handler.EditLikeAndCollect)
		like.GET("/getInfo", handler.LikeAndCollect)
	}

	router.GET("/ws", handler.WebSocket)
	router.GET("/qrcode", handler.Qrcode)
	return router
}
