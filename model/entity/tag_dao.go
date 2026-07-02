package entity

import "blog-backend/model/conn"

func GetTags(pageSize, currentPage int) ([]*Tags, int64, error) {
	var tags []*Tags
	var total int64
	if pageSize <= 0 {
		pageSize = 10
	}
	if currentPage <= 0 {
		currentPage = 1
	}
	offset := (currentPage - 1) * pageSize
	query := conn.MysqlConn.Model(&Tags{})
	if err := query.Count(&total).Error; err != nil {
		return nil, 0, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(offset).Find(&tags).Error; err != nil {
		return nil, 0, err
	}
	return tags, total, nil
}

func CreateTag(name string) error {
	tag := &Tags{Name: name, TagType: 0}
	return conn.MysqlConn.Create(tag).Error
}

func UpdateTag(id int, name string) error {
	return conn.MysqlConn.Model(&Tags{}).Where("id = ?", id).Update("name", name).Error
}

func DeleteTagsByIds(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&Tags{}, ids).Error
}
