package handler

import (
	pb "blog-backend/proto"
	"blog-backend/service"
	"github.com/gin-gonic/gin"
	"net/http"
	"strconv"
)

func EditLikeAndCollect(c *gin.Context) {
	req := pb.LikeOrCollectRequest{}
	resp := pb.BaseResp{}
	token, exist := c.Get("user")
	if !exist {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, "")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	s := token.(string)
	userInfo, err := service.ParseToken(s)
	if err != nil {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	if err := c.Bind(&req); err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	err = service.UpdateLikeOrCollect(userInfo.ID, int(req.Id), req.IsLike, req.Flag)
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}

func LikeAndCollect(c *gin.Context) {
	article_id, exist := c.GetQuery("id")
	resp := pb.IsLikeOrCollectResp{}
	if !exist {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "Need id!")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	token, exist := c.Get("user")
	if !exist {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, "")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	s := token.(string)
	userInfo, err := service.ParseToken(s)
	if err != nil {
		resp.Code = uint32(AuthError)
		resp.Msg = ConvertMsg(AuthError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	atoi, err := strconv.Atoi(article_id)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "Id Error!")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	article, err := service.GetOne(atoi)
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, "article not exist!")
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	resp.Like, resp.Collect, err = service.GetLikeAndCollect(userInfo.ID, int(article.Obj.XId))
	if err != nil {
		resp.Code = uint32(DbError)
		resp.Msg = ConvertMsg(DbError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	c.ProtoBuf(http.StatusOK, &resp)
}
