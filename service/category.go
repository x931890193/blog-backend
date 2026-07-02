package service

import (
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"strconv"
)

func AddCategory(title, des string, support bool) (*entity.Category, error) {
	category := entity.Category{Name: title, DisplayName: title, SeoDesc: des, Support: support}
	one, err := category.AddOneCategory()
	if err != nil {
		return nil, err
	}
	return one, nil
}

func GetCategoryMap() (map[int]*entity.Category, error) {
	category := entity.Category{}
	list, err := category.GetAllCategory(0, 0)
	if err != nil {
		return nil, err
	}
	res := map[int]*entity.Category{}
	for _, l := range list {
		res[l.ID] = l
	}
	return res, nil
}

func GetCategoryList(pageSize, currentPage int, title, description string) (*pb.AdminCategoryListResp, error) {
	list, total, err := entity.GetAdminCategoryList(pageSize, currentPage, title, description)
	if err != nil {
		return nil, err
	}
	resp := &pb.AdminCategoryListResp{Rows: []*pb.AdminCategoryListRespCategoryBase{}}
	resp.Total = uint32(total)
	article := entity.Article{}
	for _, l := range list {
		row := &pb.AdminCategoryListRespCategoryBase{
			Id:          uint32(l.ID),
			Title:       l.Name,
			Description: l.SeoDesc,
			CreateTime:  l.CreatedAt.Format("2006-01-02 15:04:05"),
			Support:     l.Support,
			BlogList:    []*pb.AdminCategoryListRespBlogBase{},
		}
		articles, err := article.GetAllArticle(uint(l.ID))
		if err != nil {
			return nil, err
		}
		for _, item := range articles {
			row.BlogList = append(row.BlogList, &pb.AdminCategoryListRespBlogBase{
				Title:      item.Title,
				Summary:    item.Summary,
				HeaderImg:  item.HeaderImg,
				Comment:    strconv.FormatBool(item.CanComment),
				Weight:     uint32(item.Weight),
				Support:    item.Support,
				CreateTime: item.CreatedAt.Format("2006-01-02 15:04:05"),
			})
		}
		resp.Rows = append(resp.Rows, row)
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

func DeleteCategories(ids []int) error {
	return entity.DeleteCategoriesByIds(ids)
}
