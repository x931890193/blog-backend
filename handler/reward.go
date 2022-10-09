package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
)

const (
	aliPay    = "https://cdn.mongona.com/static/images/alipay.jpg"
	wechatPay = "https://cdn.mongona.com/static/images/wechat.jpg"
)

func RewardList(c *gin.Context) {
	resp := pb.RewardResp{}
	resp.WechatImage = wechatPay
	resp.AliPayImage = aliPay
	rewards, err := service.GetWards()
	if err != nil {
		return
	}
	resp.Rewards = rewards
	c.ProtoBuf(http.StatusOK, &resp)
}
