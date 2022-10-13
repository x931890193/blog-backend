package mail

import (
	"fmt"
	"sync"
	"testing"
)

func TestSendEmail(t *testing.T) {
	wg := &sync.WaitGroup{}
	tplComment := Comment{
		Site:          "dsadsadsada",
		Username:      "sssss",
		Who:           "大哥大",
		Time:          "2022.08.25",
		OriginComment: "我是一个大傻逼",
		NewComment:    "你才是大傻逼吧",
		Url:           "https://www.bytealien.com/",
	}
	newUpdate := NewUpdate{
		Username: "狗蛋",
		Site:     "site",
		Time:     "2022.08.15",
		Title:    "哔哩哔哩源码分析",
		Summary:  "哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析哔哩哔哩源码分析",
		Url:      "https://www.bytealien.com/",
	}
	fmt.Println(newUpdate)

	for i := 0; i < 1; i++ {
		wg.Add(1)
		go SendEmail([]string{"ysudqfs@163.com"}, "new_comment", tplComment, wg)
	}
	wg.Wait()
}
