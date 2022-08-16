package handler

import (
	"blog-backend/config"
	"blog-backend/logger"
	pb "blog-backend/proto"
	"blog-backend/service"
	"blog-backend/utils/qiniu"
	"bufio"
	"errors"
	"fmt"
	"github.com/gin-gonic/gin"
	"io"
	"net/http"
	"strings"
)

func UploadFile(c *gin.Context) {
	//qiniu.UploadStream()
	resp := pb.UploadFileResp{}
	f, file, err := c.Request.FormFile("file")
	if err != nil {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, err.Error())
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	// 限制文件上传大小
	if float64(file.Size)/1024/1024 > 10 {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, errors.New("file to large！"))
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	nameArr := strings.Split(file.Filename, ".")
	suffixName := nameArr[len(nameArr)-1]
	// 验证 是否支持该格式上传,懒人写法
	suffixArr := []string{"xlsx", "txt", "jpg", "png", "jpeg", "doc", "docx", "ppt", "pptx", "xls", "mp3", "mp4", "exe"}
	var indexStr string
	for i := 0; i < len(suffixArr); i++ {
		if suffixArr[i] == suffixName {
			indexStr = "." + suffixArr[i]
		}
	}
	if indexStr == "" {
		resp.Code = uint32(ParamsError)
		resp.Msg = ConvertMsg(ParamsError, errors.New(fmt.Sprintf("not support:  %v", suffixName)))
		c.ProtoBuf(http.StatusOK, &resp)
		return
	}
	rd := bufio.NewReader(f)
	buf := []byte{}
	for {
		line, err := rd.ReadBytes('\n')
		buf = append(buf, line...)
		if err == io.EOF {
			logger.Logger.Infof("%s文件读完", file.Filename)
			break
		}
		if err != nil {
			logger.Logger.Infof("%s文件读完", err.Error())
			return
		}
	}
	resource, err := service.CheckResourceMa5(buf)
	if resource != nil && err != nil {
		resp.Url = config.Cfg.Qiniu.Host + resource.Key
	} else {
		res := qiniu.UploadStream(file.Filename, buf)
		resp.Url = config.Cfg.Qiniu.Host + res.Key
		resource.Key = res.Key
		err := resource.Save()
		if err != nil {
			resp.Code = uint32(LogicError)
			resp.Msg = ConvertMsg(LogicError, err.Error())
		}
	}

	c.ProtoBuf(http.StatusOK, &resp)
}
