package logger

import (
	"blog-backend/config"
	"github.com/sirupsen/logrus"
	"go.elastic.co/ecslogrus"
	"gopkg.in/natefinch/lumberjack.v2"
	"os"
	"path/filepath"
)

var Logger *logrus.Entry

func init() {
	fileName := filepath.Join(config.BasePath, "log")
	log := logrus.New()

	if os.Getenv("PROGRAM_ENV") == "prod" {
		logger := &lumberjack.Logger{
			// 日志输出文件路径
			Filename: fileName,
			// 日志文件最大 size, 单位是 MB
			MaxSize: 300, // megabytes
			// 最大过期日志保留的个数
			MaxBackups: 30,
			// 保留过期文件的最大时间间隔,单位是天
			// 是否需要压缩滚动日志, 使用的 gzip 压缩
			Compress: true, // disabled by default
		}
		log.SetFormatter(&ecslogrus.Formatter{})
		log.SetOutput(logger)
	} else {
		log.SetFormatter(&logrus.TextFormatter{
			DisableColors: false,
			FullTimestamp: true,
		})
	}
	Logger = log.WithField("psm", "blog")
}
