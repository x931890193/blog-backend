package utils

import (
	"blog-backend/config"
	"math/rand"
)

func GetRandomTag() string {
	return config.UserTags[rand.Intn(len(config.UserTags))]
}
