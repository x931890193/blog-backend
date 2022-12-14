package cache

import (
	"blog-backend/config"
	"fmt"
	"github.com/go-redis/redis/v7"
	"os"
)

var Client *redis.Client

func init() {
	conf := config.Cfg
	host := conf.Cache.Host
	if os.Getenv("PROGRAM_ENV") == "prod" {
		host = "redis"
	}
	Client = redis.NewClient(&redis.Options{
		Addr:     fmt.Sprintf("%v:%v", host, conf.Cache.Port),
		Password: conf.Cache.PassWord,
		DB:       conf.Cache.Db,
	})
	_, err := Client.Ping().Result()
	if err != nil {
		panic("Failed to ping redis, err:" + err.Error())

	}
}
