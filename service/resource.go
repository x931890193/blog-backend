package service

import (
	"blog-backend/model/entity"
	"blog-backend/utils/crypt"
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
