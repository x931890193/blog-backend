package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
)

func AddCategory(title, des string, support bool) (*entity.Category, error) {
	category := entity.Category{Name: title, DisplayName: title, SeoDesc: des, Support: support}
	one, err := category.AddOneCategory()
	if err != nil {
		return nil, err
	}
	return one, nil
}

func GetCategoryList(pageSize, currentPage int) (*pb.AdminCategoryListResp, error) {
	category := entity.Category{}
	list, err := category.GetAllCategory(pageSize, currentPage)
	if err != nil {
		return nil, err
	}
	resp := &pb.AdminCategoryListResp{Rows: []*pb.AdminCategoryListRespCategoryBase{}}
	resp.Total = uint32(len(list))
	for _, l := range list {
		resp.Rows = append(resp.Rows, &pb.AdminCategoryListRespCategoryBase{
			Id:          uint32(l.ID),
			Title:       l.Name,
			Description: l.SeoDesc,
			CreateTime:  l.CreatedAt.Format("2006-01-02 15:04:05"),
			Support:     l.Support,
			BlogList:    []*pb.AdminCategoryListRespBlogBase{},
		})
	}
	return resp, nil
}

func UpdateCategoryById(id int, update *pb.AdminEditCategoryRequest) error {
	category := entity.Category{}
	category.ID = id
	category.Name = update.Title
	category.DisplayName = update.Description
	category.SeoDesc = update.Description
	err := category.UpdateById()
	if err != nil {
		return err
	}
	return nil
}
