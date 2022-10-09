package handler

import (
	"github.com/gin-gonic/gin"
	"strings"
)

func Qrcode(c *gin.Context) {
	if strings.Contains(c.Request.UserAgent(), "MicroMessenger") {
		// not work wechat
		c.Redirect(302, "https://mp.weixin.qq.com/a/~~c91zvEl0ApY~Bf0lHJjI3u2q_qThjRgrJA~~")
	} else if strings.Contains(c.Request.UserAgent(), "Alipay") {
		c.Redirect(302, "https://qr.alipay.com/fkx15927h5q2xjbpvvdko14")
	}
}
