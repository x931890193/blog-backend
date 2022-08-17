package mail

import (
	"fmt"
	"sync"
	"testing"
)

func TestSendEmail(t *testing.T) {
	wg := &sync.WaitGroup{}
	tplComment := map[string]string{
		"site":           "dsadsadsada",
		"username":       "sssss",
		"who":            "大哥大",
		"time":           "2022.08.25",
		"origin_comment": "我是一个大傻逼",
		"new_comment":    "你才是大傻逼吧",
		"url":            "https://www.baidu.com",
	}
	fmt.Println(tplComment)
	newUpdate := NewUpdate{
		Username: "狗蛋",
		Site:     "site",
		Time:     "2022.08.15",
		Title:    "哔哩哔哩源码分析",
		Summary:  "哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析",
		Url:      "https://www.baidu.com",
	}
	for i := 0; i < 1; i++ {
		wg.Add(1)
		go SendEmail([]string{"ysudqfs@163.com"}, newUpdate.Title, newUpdate, wg)
	}
	wg.Wait()
}
