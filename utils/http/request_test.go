package utils

import (
	"blog-backend/logger"
	"fmt"
	"testing"
)

func TestRequest(t *testing.T) {
	data, err := Get("https://www.mongona.com", map[string]string{"s": "nginx"}, 1, nil)
	if err != nil {
		return
	}
	fmt.Println(string(data))
	logger.Logger.Error(1111)
}
