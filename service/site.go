package service

import pb "blog-backend/proto"

func GetInfo() (*pb.SiteInfoResp, error) {
	return &pb.SiteInfoResp{
		Author: "stao",
		Github: "https://github.com/x931890193",
		Beian:  "渝ICP备18016573号-1",
	}, nil
}
