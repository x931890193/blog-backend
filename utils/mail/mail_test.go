package mail

import (
	"fmt"
	"os"
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

	tpl2 := map[string]string{
		"username": "狗蛋",
		"site":     "site",
		"time":     "2022.08.15",
		"title":    "哔哩哔哩源码分析",
		"summary":  "哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析",
		"url":      "https://www.baidu.com",
	}
	body := os.Expand(NewUpdateTpl, func(k string) string { return tpl2[k] })
	for i := 0; i <= 1; i++ {
		wg.Add(1)
		go SendEmail([]string{"ysudqfs@163.com"}, "123344", body, wg)
	}
	wg.Wait()
}
