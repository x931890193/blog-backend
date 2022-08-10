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
	//router.Use(middleware.RequestMiddleware())
	admin := router.Group("/admin")
	{
		admin.POST("/generate", handler.GenerateAdmin)
		admin.GET("/captcha", handler.GetCaptcha)
		admin.POST("/login", handler.AdminLogin)
		admin.Use(middleware.AuthMiddleware())
		admin.GET("/info", handler.AdminInfo)
		admin.GET("/routers", handler.Routers)
		admin.POST("/logout", handler.LoginOut)
		admin.POST("/article/category/add", handler.AddCategory)
		admin.POST("/article/category/edit", handler.LoginOut)
		admin.GET("/article/category/list", handler.AdminCategoryList)
	}
	// article
	article := router.Group("/article")
	{
		article.GET("/list", handler.GetArticleList)
		article.GET("/getListByClass") // 归档标签
		article.GET("/getInfo")
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
		resource.GET("/site_info")
		resource.GET("/about")
	}
	// user
	user := router.Group("/user")
	{
		user.GET("")
		user.POST("/login")
		user.POST("/logout")
		user.GET("/getUserInfo")
	}
	// reward
	reward := router.Group("/reward")
	{
		reward.GET("/list")
	}
	// links
	links := router.GET("/links")
	{
		links.GET("/list")
	}

	return router
}
