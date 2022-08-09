package captcha

import (
	"blog-backend/cache"
	"strconv"
	"time"
)

const captchaPrefix = "captcha:"

type RedisStore struct {
}

func (r RedisStore) Set(id string, value string) error {
	v, _ := strconv.Atoi(value)
	cache.Client.Set(captchaPrefix+id, v, time.Second*180)
	return nil
}

func (r RedisStore) Get(id string, clear bool) string {
	key := captchaPrefix + id
	re, err := cache.Client.Get(key).Result()
	if err != nil {
		return ""
	}

	if clear {
		cache.Client.Del(key)
	}

	return re
}

func (r RedisStore) Verify(id, answer string, clear bool) bool {
	v := RedisStore{}.Get(id, clear)
	return v == answer
}
