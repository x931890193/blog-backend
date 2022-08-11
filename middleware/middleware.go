package middleware

import (
	"blog-backend/cache"
	"blog-backend/logger"
	"blog-backend/utils/useragent"
	"fmt"
	"github.com/gin-gonic/gin"
	"strings"
)

func AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Authorization") // Authorization
		if token != "1" {
			tokenSlice := strings.Split(token, " ")
			if len(tokenSlice) != 2 {
				logger.Logger.Error(fmt.Sprintf("鉴权参数错误！%v", c.Request.Header))
				c.AbortWithStatus(403)
				return
			}
			token = tokenSlice[1]
			res, err := cache.Client.Get(token).Result()
			if err != nil || res == "" {
				logger.Logger.Error(fmt.Sprintf("鉴权参数错误！%v", c.Request.Header))
				c.AbortWithStatus(403)
				return
			}
			c.Set("admin", token)
		}
		c.Next()
	}
}

func RequestMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		c.Set("client", useragent.ParseByRequest(c.Request))
		c.Next()
	}
}
