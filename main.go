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
	"time"
)

func initCrontabTask() {
	c := cron.New()
	spec := "0 */24 * * * ?"
	if err := c.AddFunc(spec, crontab.SaveAliOrder); err != nil {
		logger.Logger.Error(err.Error())
	}
	if err := c.AddFunc(spec, crontab.SaveWechatOrder); err != nil {
		logger.Logger.Error(err.Error())
	}
	c.Start()
	select {}
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
