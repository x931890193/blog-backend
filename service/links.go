package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
)

func AddLink(req *pb.LinkRequest) error {
	link := entity.Link{
		Title:       req.Title,
		Description: req.Description,
		Email:       req.Email,
		Url:         req.Url,
		HeaderImg:   req.HeaderImg,
	}
	err := link.AddOne()
	if err != nil {
		return err
	}
	return nil
}

func LinkList() (*pb.LinkListResp, error) {
	link := entity.Link{}
	list, err := link.GetAllList()
	if err != nil {
		return nil, err
	}
	item := []*pb.LinkBase{}
	for _, l := range list {
		item = append(item, &pb.LinkBase{
			Title:       l.Title,
			Description: l.Description,
			Email:       l.Email,
			Url:         l.Url,
			HeaderImg:   l.HeaderImg,
			Display:     l.IsDelete,
			Id:          uint32(l.ID),
			CreateTime:  l.CreatedAt.Format("2006-01-02 15:04:05"),
		})
	}
	return &pb.LinkListResp{
		Total: uint32(len(item)),
		Rows:  item,
	}, nil
}
