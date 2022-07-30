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
	r := router.Group("/api")
	{
		admin := r.Group("/admin")
		{
			admin.POST("/generate", handler.GenerateAdmin)
			admin.POST("/login", handler.AdminLogin)
			admin.Use(middleware.AuthMiddleware())
			admin.POST("/logout", handler.LoginOut)
			admin.GET("/info", handler.AdminInfo)

		}
		// article
		article := r.Group("/article")
		{
			article.GET("/")
			article.GET("/list")
		}
		// comment
		comment := r.Group("/comment")
		{
			comment.GET("/getTopComment")
		}
		// resource
		resource := r.Group("/resource")
		{
			resource.GET("/about")
		}
		// user
		user := r.Group("/user")
		{
			user.GET("")
		}
		// reward
		reward := r.Group("/reward")
		{
			reward.GET("/list")
		}
		// links
		links := r.GET("/links")
		{
			links.GET("/list")
		}
	}

	return router
}
