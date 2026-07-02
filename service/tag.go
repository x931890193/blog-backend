package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
)

func GetTagList(pageSize, currentPage int) (*pb.ListByClassResp, int64, error) {
	tags, total, err := entity.GetTags(pageSize, currentPage)
	if err != nil {
		return nil, 0, err
	}
	resp := &pb.ListByClassResp{Tags: []*pb.Tags{}}
	for _, tag := range tags {
		resp.Tags = append(resp.Tags, &pb.Tags{
			XId:  uint32(tag.ID),
			Name: tag.Name,
		})
	}
	return resp, total, nil
}

func AddTag(name string) error {
	return entity.CreateTag(name)
}

func UpdateTag(id int, name string) error {
	return entity.UpdateTag(id, name)
}

func DeleteTags(ids []int) error {
	return entity.DeleteTagsByIds(ids)
}
