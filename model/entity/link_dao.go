package entity

import (
	"blog-backend/model/conn"
	"errors"
	"fmt"
	"strings"

	"gorm.io/gorm"
)

const (
	VerifyInt = iota
	VerifyPass
	VerifyFailed
)

func (l *Link) AddOne() error {
	if err := conn.MysqlConn.Model(l).Create(&l).Error; err != nil {
		return err
	}
	return nil
}

func (l *Link) GetAllList() ([]*Link, error) {
	res := []*Link{}
	if err := conn.MysqlConn.Model(l).Where("is_delete = ?", false).Find(&res).Error; err != nil {
		return nil, err
	}
	return res, nil
}

func GetLinkList(admin bool, pageSize, currentPage int, title, description, beginTime, endTime string) ([]*Link, int64, error) {
	res := []*Link{}
	var total int64
	query := conn.MysqlConn.Model(&Link{}).Where("is_delete = ?", false)
	if !admin {
		query = query.Where("verify_status = ? and show_link = ?", VerifyPass, true)
	}
	if v := strings.TrimSpace(title); v != "" {
		query = query.Where("title LIKE ?", "%"+v+"%")
	}
	if v := strings.TrimSpace(description); v != "" {
		query = query.Where("description LIKE ?", "%"+v+"%")
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
	if pageSize > 0 && currentPage > 0 {
		query = query.Limit(pageSize).Offset((currentPage - 1) * pageSize)
	}
	if err := query.Order("created_at DESC").Find(&res).Error; err != nil {
		return nil, 0, err
	}
	return res, total, nil
}

func (l *Link) GetLinByUserId() error {
	if err := conn.MysqlConn.Model(l).Where("user_id = ? and is_delete = ?", l.UserId, false).First(l).Error; err != nil && err != gorm.ErrRecordNotFound {
		return err
	}
	return nil
}

func (l *Link) UpdateOrCreate(v map[string]interface{}) error {
	existing := Link{}
	err := conn.MysqlConn.Model(&Link{}).Where("user_id = ?", l.UserId).First(&existing).Error
	if errors.Is(err, gorm.ErrRecordNotFound) {
		l.Title, _ = v["title"].(string)
		l.Description, _ = v["description"].(string)
		l.Email, _ = v["email"].(string)
		if l.Email == "" {
			l.Email = fmt.Sprintf("user-%d@bytealien.link", l.UserId)
		}
		l.Url, _ = v["url"].(string)
		l.HeaderImg, _ = v["header_img"].(string)
		l.ShowLink, _ = v["show_link"].(bool)
		if status, ok := v["verify_status"].(int); ok {
			l.VerifyStatus = status
		}
		if err := conn.MysqlConn.Create(l).Error; err != nil {
			return err
		}
		return nil
	}
	if err != nil {
		return err
	}
	v["is_delete"] = false
	if err := conn.MysqlConn.Model(&Link{}).Where("id = ?", existing.ID).UpdateColumns(v).Error; err != nil {
		return err
	}
	if err := conn.MysqlConn.First(l, existing.ID).Error; err != nil {
		return err
	}
	return nil
}

func (l *Link) UpdateDisplay(display bool) error {
	return conn.MysqlConn.Model(l).Where("id = ?", l.ID).Update("show_link", display).Error
}

func (l *Link) GetByID() error {
	return conn.MysqlConn.Model(l).Where("id = ? and is_delete = ?", l.ID, false).First(l).Error
}

func (l *Link) UpdateByID(values map[string]interface{}) error {
	return conn.MysqlConn.Model(l).Where("id = ? and is_delete = ?", l.ID, false).UpdateColumns(values).Error
}

func DeleteLinksByIds(ids []int) error {
	if len(ids) == 0 {
		return nil
	}
	return conn.MysqlConn.Model(&Link{}).Where("id in (?)", ids).Update("is_delete", true).Error
}

func (l *Link) GetLinkStatus() string {
	return map[int]string{
		VerifyInt:    "审核中",
		VerifyPass:   "审核通过",
		VerifyFailed: "审核不通过",
	}[l.VerifyStatus]
}
