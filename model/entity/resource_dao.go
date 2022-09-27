package entity

import (
	"blog-backend/model/conn"
	"gorm.io/gorm"
)

func (r *Resource) GetByUuid() error {
	if err := conn.MysqlConn.Model(r).Where("uuid=?", r.Uuid).Find(r).Error; err != nil {
		return err
	}
	return nil
}

func (r *Resource) Save() error {
	if err := conn.MysqlConn.Model(r).Create(r).Error; err != nil {
		return err
	}
	return nil
}

func (s *SiteInfo) Get() error {
	if err := conn.MysqlConn.Model(s).First(&s).Error; err != gorm.ErrRecordNotFound {
		return err
	}
	return nil
}

// UpdateOrCreate Assign amazing!!!!
func (s *SiteInfo) UpdateOrCreate(v map[string]interface{}) error {
	if err := conn.MysqlConn.Model(s).Where("id=?", s.ID).Assign(v).FirstOrCreate(&s).Error; err != nil {
		return err
	}
	return nil
}
