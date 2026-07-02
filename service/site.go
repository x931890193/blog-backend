package service

import pb "blog-backend/proto"

func GetInfo() (*pb.SiteInfoResp, error) {
	return &pb.SiteInfoResp{
		Author: "stao",
		Github: "https://github.com/x931890193",
		Beian:  "渝ICP备18016573号-1",
	}, nil
}

func DefaultSelfDescription() string {
	return `## 简单背景

- 94 年诞生
- 重庆巴南，目前北漂
- 本科 [燕山大学](https://www.ysu.edu.cn/)
- 后端开发工程师
- 技术宅一枚，喜欢折腾各种电子产品、钻研各种技术，也喜欢骑行、健身等运动
- 精通各种语言的 Hello World
- 我的理想：**世界和平！**
- 我的简历：[http://resume.mongona.com/](http://resume.mongona.com/)
- Email: ysudqfs#163.com

## 关于这个站

这里会长期记录后端开发、AI 工程化、产品想法和个人项目维护过程。欢迎留言、交流技术问题，也欢迎交换友链。`
}
