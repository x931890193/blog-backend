package middleware

import (
	"blog-backend/cache"
	"blog-backend/logger"
	"crypto/hmac"
	"crypto/sha1"
	"encoding/base64"
	"fmt"
	"github.com/gin-gonic/gin"
	"strings"
)

func AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Token") // Authorization
		if token != "1" {
			res := cache.Client.Get(token)
			if res == nil {
				logger.Logger.Error(fmt.Sprintf("鉴权参数错误！%v", c.Request.Header))
				c.AbortWithStatus(403)
				return
			}
		}
		c.Next()
		println(111111)
	}
}

func RequestMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Token") // Authorization
		if token != "1" {
			signature := c.Request.Header.Get("Signature")
			authorize := c.Request.Header.Get("X-authorize-uuid") // 时间戳：随机uuid
			paramsSlice := strings.Split(authorize, ":")
			if len(paramsSlice) != 2 {
				logger.Logger.Error(fmt.Sprintf("鉴权参数错误！%v", c.Request.Header))
				c.AbortWithStatus(403)
				return
			}
			secret := "Rtg8BPKNEf2mB4mgvKONGPZZQSaJWNLijxR42qRgq0iBb5"
			key := []byte(secret)
			h := hmac.New(sha1.New, key)
			h.Write([]byte(authorize))
			sign := base64.StdEncoding.EncodeToString([]byte(h.Sum(nil)))
			if sign != signature {
				logger.Logger.Error(fmt.Sprintf("签名错误！%v", c.Request.Header))
				c.AbortWithStatus(403)
				return
			}

		}
		c.Next()
	}
}
