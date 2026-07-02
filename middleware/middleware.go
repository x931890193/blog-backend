package middleware

import (
	"blog-backend/cache"
	"blog-backend/logger"
	"blog-backend/model/entity"
	"blog-backend/service"
	"blog-backend/utils/useragent"
	"fmt"
	"github.com/gin-gonic/gin"
	"net"
	"strings"
	"time"
)

func BaseAuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Authorization") // Authorization
		setUser := true
		tokenSlice := strings.Split(token, " ")
		if len(tokenSlice) != 2 {
			setUser = false
		} else {
			token = tokenSlice[1]
			res, err := cache.Client.Get(token).Result()
			if err != nil || res == "" {
				setUser = false
			}
		}
		if setUser {
			c.Set("user", token)
		}
		c.Next()
	}
}

func AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Authorization") // Authorization
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
		userInfo, err := service.ParseToken(token)
		if err != nil || !userInfo.IsAdmin {
			logger.Logger.Error(fmt.Sprintf("鉴权参数错误！%v", c.Request.Header))
			c.AbortWithStatus(403)
			return
		}
		c.Set("admin", token)
		c.Next()
	}
}

func RequestMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		start := time.Now()
		client := useragent.ParseByRequest(c.Request)
		realIP := realClientIP(c)
		client.Ip = realIP
		c.Set("client", client)
		c.Next()
		if c.Request.URL.Path == "/ws" {
			return
		}
		username := ""
		if tokenValue, ok := c.Get("admin"); ok {
			if userInfo, err := service.ParseToken(tokenValue.(string)); err == nil {
				username = userInfo.UserName
			}
		} else if tokenValue, ok := c.Get("user"); ok {
			if userInfo, err := service.ParseToken(tokenValue.(string)); err == nil {
				username = userInfo.UserName
			}
		}
		errMsg := ""
		if len(c.Errors) > 0 {
			errMsg = c.Errors.String()
		}
		reqLog := &entity.Request{
			IP:          realIP,
			Referer:     c.Request.Referer(),
			URL:         c.Request.URL.Path,
			RemoteAddr:  realIP,
			UserAgent:   client,
			OpType:      requestOpType(c.Request.Method),
			Method:      c.Request.Method,
			IsLogin:     isLoginRequest(c.Request.URL.Path),
			RequestTime: uint(time.Since(start).Milliseconds()),
			StatusCode:  c.Writer.Status(),
			UserName:    username,
			ErrorMsg:    errMsg,
		}
		go func() {
			if err := service.SaveAdminRequestLog(reqLog); err != nil {
				logger.Logger.Error(fmt.Sprintf("保存请求日志失败: %v", err))
			}
		}()
	}
}

func realClientIP(c *gin.Context) string {
	headers := []string{
		"X-Forwarded-For",
		"X-Original-Forwarded-For",
		"X-Real-IP",
		"X-Client-IP",
		"CF-Connecting-IP",
		"True-Client-IP",
	}
	candidates := make([]string, 0, len(headers)+2)
	for _, header := range headers {
		for _, part := range strings.Split(c.GetHeader(header), ",") {
			if ip := normalizeIP(part); ip != "" {
				candidates = append(candidates, ip)
			}
		}
	}
	candidates = append(candidates, normalizeIP(c.ClientIP()))
	candidates = append(candidates, normalizeIP(c.Request.RemoteAddr))
	for _, ip := range candidates {
		if !isLoopbackIP(ip) {
			return ip
		}
	}
	for _, ip := range candidates {
		if ip != "" {
			return ip
		}
	}
	return ""
}

func normalizeIP(value string) string {
	value = strings.TrimSpace(value)
	if value == "" {
		return ""
	}
	if host, _, err := net.SplitHostPort(value); err == nil {
		value = host
	}
	value = strings.Trim(value, "[]")
	parsed := net.ParseIP(value)
	if parsed == nil {
		return ""
	}
	return parsed.String()
}

func isLoopbackIP(value string) bool {
	ip := net.ParseIP(value)
	return ip == nil || ip.IsLoopback() || ip.IsUnspecified()
}

func isLoginRequest(path string) bool {
	path = strings.ToLower(strings.TrimSpace(path))
	return strings.Contains(path, "/login") ||
		strings.Contains(path, "/logout") ||
		strings.Contains(path, "/oauth")
}

func requestOpType(method string) string {
	switch method {
	case "POST":
		return "1"
	case "PUT", "PATCH":
		return "2"
	case "DELETE":
		return "3"
	default:
		return "0"
	}
}
