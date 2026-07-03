package main

import (
	"blog-backend/config"
	"blog-backend/crontab"
	"blog-backend/logger"
	_ "blog-backend/model/conn"
	_ "blog-backend/model/entity"
	"blog-backend/router"
	"context"
	"fmt"
	"github.com/robfig/cron"
	"net/http"
	"os"
	"os/signal"
	"strings"
	"time"
)

func initCrontabTask() {
	c := cron.New()
	spec := "0 */24 * * * ?"
	registerCronFunc(c, "save_alipay_order", spec, crontab.SaveAliOrder)
	registerCronFunc(c, "save_wechat_order", spec, crontab.SaveWechatOrder)
	if config.Cfg.AIArticle.Enabled {
		aiArticleSpec := config.Cfg.AIArticle.Spec
		if aiArticleSpec == "" {
			aiArticleSpec = "0 37 6 * * ?"
		}
		if strings.TrimSpace(config.Cfg.AIArticle.APIKey) == "" {
			logger.Logger.Warn("AIArticle cron enabled but api_key is empty; generation will be skipped")
		}
		registerCronFunc(c, "ai_article", aiArticleSpec, crontab.GenerateAIArticle)
	} else {
		logger.Logger.Info("AIArticle cron disabled")
	}
	c.Start()
	logger.Logger.Info("cron scheduler started")
	select {}
}

func registerCronFunc(c *cron.Cron, name, spec string, fn func()) {
	if err := c.AddFunc(spec, fn); err != nil {
		logger.Logger.Error(fmt.Sprintf("cron register failed: name=%s spec=%s error=%s", name, spec, err.Error()))
		return
	}
	logger.Logger.Info(fmt.Sprintf("cron registered: name=%s spec=%s next=%s", name, spec, cronNextRun(spec)))
}

func cronNextRun(spec string) string {
	schedule, err := cron.Parse(spec)
	if err != nil {
		return fmt.Sprintf("invalid spec: %s", err.Error())
	}
	next := schedule.Next(time.Now())
	if next.IsZero() {
		return "unscheduled"
	}
	return next.Format("2006-01-02 15:04:05 MST")
}

func main() {
	engine := router.SetupServer()
	conf := config.Cfg
	go initCrontabTask()
	serverUrl := fmt.Sprintf("%s:%s", conf.Server.Host, conf.Server.Port)
	server := &http.Server{Addr: serverUrl, Handler: engine}
	fmt.Println(fmt.Sprintf("server listen: http://%s", serverUrl))
	err := server.ListenAndServe()
	if err != nil {
		panic(err.Error())
	}
	// 监听系统中断信号，收到中断信号之后停止服务
	quit := make(chan os.Signal)
	signal.Notify(quit, os.Interrupt)
	<-quit
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	defer func() {
		cancel()
		logger.Logger.Info("do something!")
	}()
	if err = server.Shutdown(ctx); err != nil {
		fmt.Println(err.Error())
	}
}
