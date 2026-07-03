package entity

import (
	"blog-backend/model/conn"
	"strings"
)

func GetTags(pageSize, currentPage int, title, beginTime, endTime string) ([]*Tags, int64, error) {
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
	if v := strings.TrimSpace(title); v != "" {
		query = query.Where("name LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(beginTime); v != "" {
		query = query.Where("created_at >= ?", v+" 00:00:00")
	}
	if v := strings.TrimSpace(endTime); v != "" {
		query = query.Where("created_at <= ?", v+" 23:59:59")
	}
	if err := query.Count(&total).Error; err != nil {
		return nil, 0, err
	}
	if err := query.Order("created_at DESC").Limit(pageSize).Offset(offset).Find(&tags).Error; err != nil {
		return nil, 0, err
	}
	return tags, total, nil
}

func GetTagByID(id int) (*Tags, error) {
	tag := &Tags{}
	if err := conn.MysqlConn.First(tag, id).Error; err != nil {
		return nil, err
	}
	return tag, nil
}

func CreateTag(name, color string) error {
	tag := &Tags{Name: name, TagType: 0, Color: color}
	return conn.MysqlConn.Create(tag).Error
}

func UpdateTag(id int, name, color string) error {
	return conn.MysqlConn.Model(&Tags{}).Where("id = ?", id).Updates(map[string]interface{}{
		"name":  name,
		"color": color,
	}).Error
}

func DeleteTagsByIds(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Delete(&Tags{}, ids).Error
}
