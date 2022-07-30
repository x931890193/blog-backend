package qiniu

import (
	"blog-backend/cache"
	"blog-backend/config"
	"blog-backend/logger"
	"bytes"
	"context"
	"fmt"
	"github.com/qiniu/go-sdk/v7/auth/qbox"
	"github.com/qiniu/go-sdk/v7/storage"
	"time"
)

var (
	cfg     *storage.Config
	upToken string
	mac     *qbox.Mac
)

const qiNiuUpTokenCacheKey = "qiniu_token"

func init() {
	conf := config.Cfg
	mac = qbox.NewMac(conf.Qiniu.AccessKey, conf.Qiniu.SecretKey)
	if mac == nil {
		panic("qqqq")
	}
	putPolicy := storage.PutPolicy{
		Scope: conf.Qiniu.Bucket,
	}
	upToken = putPolicy.UploadToken(mac)
	cfg = &storage.Config{}
	// 空间对应的机房
	cfg.Zone = &storage.ZoneHuadong
	// 是否使用https域名
	cfg.UseHTTPS = true
	// 上传是否使用CDN上传加速
	cfg.UseCdnDomains = true
}

func getLastUpToken() {
	token, err := cache.Client.Get(qiNiuUpTokenCacheKey).Result()
	if err != nil || upToken == "" {
		putPolicy := storage.PutPolicy{Scope: config.Cfg.Qiniu.Bucket}
		putPolicy.Expires = 7200
		upToken = putPolicy.UploadToken(mac)
		cache.Client.Set(qiNiuUpTokenCacheKey, upToken, 7200*time.Second)
	} else {
		upToken = token
	}
}

func UploadLocalFile(key string, localFile string) {
	getLastUpToken()
	formUploader := storage.NewFormUploader(cfg)
	ret := storage.PutRet{}
	putExtra := storage.PutExtra{}
	err := formUploader.PutFile(context.Background(), &ret, upToken, key, localFile, &putExtra)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("upload to qiniu error: %v", err.Error()))
		panic(err.Error())
		return
	}
	logger.Logger.Info(fmt.Sprintf("upload to qiniu success, key: %v, hash: %v", ret.Key, ret.Hash))

}

func UploadStream(key string, data []byte) {
	getLastUpToken()
	formUploader := storage.NewFormUploader(cfg)
	ret := storage.PutRet{}
	putExtra := storage.PutExtra{}
	dataLen := int64(len(data))
	err := formUploader.Put(context.Background(), &ret, upToken, key, bytes.NewReader(data), dataLen, &putExtra)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("upload to qiniu error: %v", err.Error()))
		return
	}
	logger.Logger.Info(fmt.Sprintf("upload to qiniu success, key: %v, hash: %v", ret.Key, ret.Hash))
}
