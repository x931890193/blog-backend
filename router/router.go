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
		admin.POST("/article/add", handler.AdminAddArticle)
		admin.GET("/article/:id", handler.AdminGetArticle)
		admin.POST("/article/:id", handler.AdminEditArticle)
		admin.GET("/article/list", handler.AdminArticleList)
		admin.POST("/article/category/add", handler.AddCategory)
		admin.POST("/article/category/edit", handler.EditCategory)
		admin.GET("/article/category/list", handler.CategoryList)
		admin.POST("/link/add", handler.AddLink)
		admin.POST("/link/edit", handler.AddLink)
		admin.GET("/link/list", handler.LinkList)
		admin.GET("/about/get", handler.AdminGetSiteInfo)
		admin.POST("/about/edit", handler.AdminAddOrUpdateAbout)
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
	// user
	user := router.Group("/user")
	{
		user.GET("")
		user.GET("/github/oauth", handler.GitHubOauth)
		user.POST("/login")
		user.POST("/logout", handler.LoginOut)
		user.GET("/getUserInfo", handler.UserInfo)
		user.POST("/edit", handler.Edit)
	}
	// reward
	reward := router.Group("/reward")
	{
		reward.GET("/list")
	}
	// links
	links := router.Group("/link")
	{
		links.GET("/list", handler.LinkList)
	}
	// like
	like := router.Group("/like")
	{
		like.GET("/getInfo")
	}
	// collect
	collect := router.Group("collect")
	{
		collect.GET("/getInfo")
	}
	router.GET("/ws", handler.WebSocket)
	return router
}
