package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"fmt"
)

func GetWards() ([]*pb.RewardResp_Rewards, error) {
	reward := entity.Reward{}
	list, err := reward.GetList()
	if err != nil {
		return nil, err
	}
	res := []*pb.RewardResp_Rewards{}
	for _, r := range list {
		res = append(res, &pb.RewardResp_Rewards{
			Name:    r.Who,
			PayTime: r.CreatedAt.Format("2006-01-02 15:04:05"),
			Money:   fmt.Sprintf("%.2f RMB", r.Amount),
			Source:  r.GetPayMethodHuman(),
		})
	}
	return res, nil
}
