package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"strconv"
	"strings"
)

func AddLink(req *pb.LinkRequest) error {
	display := linkDisplayFromRequest(req)
	link := entity.Link{
		Title:        req.Title,
		Description:  req.Description,
		Email:        req.Email,
		Url:          req.Url,
		HeaderImg:    req.HeaderImg,
		ShowLink:     display,
		VerifyStatus: entity.VerifyPass,
	}
	err := link.AddOne()
	if err != nil {
		return err
	}
	return nil
}

func linkDisplayFromRequest(req *pb.LinkRequest) bool {
	if req.Display == "" {
		return req.ShowLink
	}
	display, err := strconv.ParseBool(req.Display)
	if err != nil {
		return req.ShowLink
	}
	return display
}

func LinkList(admin bool, pageSize, pageNum int, title, description, beginTime, endTime string) (*pb.LinkListResp, error) {
	list, total, err := entity.GetLinkList(
		admin,
		pageSize,
		pageNum,
		strings.TrimSpace(title),
		strings.TrimSpace(description),
		strings.TrimSpace(beginTime),
		strings.TrimSpace(endTime),
	)
	if err != nil {
		return nil, err
	}
	item := []*pb.LinkBase{}
	for _, l := range list {
		item = append(item, &pb.LinkBase{
			Title:        l.Title,
			Description:  l.Description,
			Email:        l.Email,
			Url:          l.Url,
			HeaderImg:    l.HeaderImg,
			Display:      l.ShowLink,
			Id:           uint32(l.ID),
			CreateTime:   l.CreatedAt.Format("2006-01-02 15:04:05"),
			Status:       l.VerifyStatus == entity.VerifyPass,
			VerifyStatus: l.GetLinkStatus(),
		})
	}
	return &pb.LinkListResp{
		Total: uint32(total),
		Rows:  item,
	}, nil
}

func UpdateAdminLink(req *pb.LinkRequest) error {
	link := entity.Link{BaseModel: entity.BaseModel{ID: int(req.Id)}}
	return link.UpdateByID(map[string]interface{}{
		"title":         req.Title,
		"description":   req.Description,
		"email":         req.Email,
		"url":           req.Url,
		"header_img":    req.HeaderImg,
		"show_link":     linkDisplayFromRequest(req),
		"verify_status": entity.VerifyPass,
	})
}

func DeleteLinks(ids []int) error {
	return entity.DeleteLinksByIds(ids)
}

func GetLink(userId uint32) (*entity.Link, error) {
	link := &entity.Link{UserId: uint(userId)}
	err := link.GetLinByUserId()
	if err != nil {
		return nil, err
	}
	return link, nil
}

func UpdateLink(req *pb.EditUserInfoRequest) (*entity.Link, error) {
	link := entity.Link{}
	link.UserId = uint(req.UserId)
	values := map[string]interface{}{
		"title":       req.Linkname,
		"description": req.LinkDesc,
		"url":         req.LinkUrl,
		"header_img":  req.LogoUrl,
		"show_link":   req.State,
	}
	if req.State {
		values["verify_status"] = entity.VerifyInt
	}
	err := link.UpdateOrCreate(values)
	if err != nil {
		return nil, err
	}
	return &link, nil
}

func ChangeLinkDisplay(id int, display bool) error {
	link := entity.Link{BaseModel: entity.BaseModel{ID: id}}
	return link.UpdateDisplay(display)
}

func ChangeLinkVerifyStatus(id int, pass bool) error {
	status := entity.VerifyFailed
	showLink := false
	if pass {
		status = entity.VerifyPass
		showLink = true
	}
	link := entity.Link{BaseModel: entity.BaseModel{ID: id}}
	return link.UpdateByID(map[string]interface{}{
		"verify_status": status,
		"show_link":     showLink,
	})
}
