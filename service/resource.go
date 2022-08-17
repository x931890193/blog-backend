package service

import (
	"blog-backend/cache"
	"blog-backend/model/entity"
	"blog-backend/utils/crypt"
	"encoding/json"
	"time"
)

func CheckResourceMa5(any []byte) (*entity.Resource, error) {
	md5, err := crypt.Md5(any)
	if err != nil {
		return nil, err
	}
	resource := &entity.Resource{Uuid: md5}
	err = resource.GetByUuid()
	if err != nil {
		return nil, err
	}
	return resource, nil
}

func SaveResource(resource *entity.Resource) error {
	err := resource.Save()
	if err != nil {
		return err
	}
	return nil
}

func GetSiteInfo() (*entity.SiteInfo, error) {
	s := &entity.SiteInfo{}
	var err error
	res, err := cache.Client.Get("site").Result()
	if err != nil && res != "" {
		err = json.Unmarshal([]byte(res), &s)
		if err != nil {
			return s, err
		}
	}
	err = s.Get()
	if err != nil {
		return nil, err
	}
	marshal, err := json.Marshal(s)
	if err != nil {
		return nil, err
	}
	cache.Client.Set("site", marshal, time.Hour*24)
	return s, nil
}

func UpdateOrCreate(id int, v map[string]interface{}) (*entity.SiteInfo, error) {
	s := entity.SiteInfo{BaseModel: entity.BaseModel{ID: id}}
	err := s.UpdateOrCreate(v)
	if err != nil {
		return nil, err
	}
	// update cache
	cache.Client.Del("site")
	return &s, nil
}
