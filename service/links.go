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

func LinkList(admin bool) (*pb.LinkListResp, error) {
	link := entity.Link{}
	list, err := link.GetAllList()
	if err != nil {
		return nil, err
	}
	item := []*pb.LinkBase{}
	for _, l := range list {
		if !admin && l.VerifyStatus != entity.VerifyPass {
			continue
		}
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

func GetLink(userId uint32) (*entity.Link, error) {
	link := &entity.Link{BaseModel: entity.BaseModel{ID: int(userId)}}
	err := link.GetLinByUserId()
	if err != nil {
		return nil, err
	}
	return link, nil
}

func UpdateLink(req *pb.EditUserInfoRequest) (*entity.Link, error) {
	link := entity.Link{}
	link.UserId = uint(req.UserId)
	err := link.UpdateOrCreate(map[string]interface{}{
		"title":         req.Linkname,
		"description":   req.LinkDesc,
		"url":           req.LinkUrl,
		"header_img":    req.LogoUrl,
		"show_link":     req.State,
		"verify_status": entity.VerifyInt,
	})
	if err != nil {
		return nil, err
	}
	return &link, nil
}
