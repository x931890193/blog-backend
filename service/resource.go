package service

import (
	"blog-backend/cache"
	"blog-backend/model/entity"
	"blog-backend/utils/crypt"
	"encoding/json"
	"errors"
	"strconv"
	"time"
)

const defaultSiteLoveCount = 99999

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
	if err == nil && res != "" {
		err = json.Unmarshal([]byte(res), &s)
		if err != nil {
			return s, err
		}
		return s, nil
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

func SiteLoveCount(siteInfo *entity.SiteInfo) int {
	if siteInfo == nil || siteInfo.LoveCount <= 0 {
		return defaultSiteLoveCount
	}
	return siteInfo.LoveCount
}

func SiteLoveCountString(siteInfo *entity.SiteInfo) string {
	return strconv.Itoa(SiteLoveCount(siteInfo))
}

func IncreaseSiteLoveCount() (int, error) {
	siteInfo, err := GetSiteInfo()
	if err != nil {
		return 0, err
	}
	id := siteInfo.ID
	if id == 0 {
		id = 1
	}
	next := SiteLoveCount(siteInfo) + 1
	_, err = UpdateOrCreate(id, map[string]interface{}{
		"love_count": next,
	})
	if err != nil {
		return 0, err
	}
	return next, nil
}

func GetPanelGroup() (interface{}, error) {
	articles, err := GetBlogList()
	if err != nil {
		return nil, err
	}
	users, err := GetUserList()
	if err != nil {
		return nil, err
	}
	requestInfos, err := GetVisitorList()
	if err != nil {
		return nil, err
	}
	return map[string]int{
		"userCount":    len(users),
		"visitorCount": len(requestInfos),
		"blogCount":    len(articles),
	}, nil
}

func getLineChartData(typeData string) (interface{}, error) {
	res := []*entity.BaseModel{}
	switch typeData {
	case "visitor":
		data, err := GetVisitorList()
		if err != nil {
			return nil, err
		}
		for _, d := range data {
			res = append(res, &entity.BaseModel{CreatedAt: d.CreatedAt})
		}
		return res, nil
	case "blog":
		data, err := GetBlogList()
		if err != nil {
			return nil, err
		}
		for _, d := range data {
			res = append(res, &entity.BaseModel{CreatedAt: d.CreatedAt})
		}
		return res, nil
	case "user":
		data, err := GetUserList()
		if err != nil {
			return nil, err
		}
		for _, d := range data {
			res = append(res, &entity.BaseModel{CreatedAt: d.CreatedAt})
		}
		return res, nil
	default:
		return nil, errors.New("need Type~")
	}
}

func GetLineChartData(typeData string) (interface{}, error) {
	d, err := getLineChartData(typeData)
	if err != nil {
		return nil, err
	}
	data, _ := d.([]*entity.BaseModel)
	dataMap := map[string]uint32{}
	for _, item := range data {
		dateTime := item.CreatedAt.Format("2006-01-02")
		dataMap[dateTime]++
	}
	axisData := []string{}
	actualData := []uint32{}
	expectedData := []uint32{}
	now := time.Now()
	for i := 6; i >= 0; i-- {
		day := now.AddDate(0, 0, -i)
		key := day.Format("2006-01-02")
		prevKey := day.AddDate(0, 0, -7).Format("2006-01-02")
		axisData = append(axisData, day.Format("01-02"))
		actualData = append(actualData, dataMap[key])
		expectedData = append(expectedData, dataMap[prevKey])
	}

	return map[string]interface{}{
		"AxisData":     axisData,
		"ExpectedData": expectedData,
		"ActualData":   actualData,
	}, nil
}

func GetUserList() ([]*entity.User, error) {
	user := entity.User{}
	users, err := user.GetListByQuery(map[string]interface{}{"is_delete": false})
	if err != nil {
		if err != nil {
			return nil, err
		}
	}
	return users, nil

}

func GetBlogList() ([]*entity.Article, error) {
	article := entity.Article{}
	articles, err := article.GetListByQuery(map[string]interface{}{"is_delete": false})

	if err != nil {
		if err != nil {
			return nil, err
		}
	}
	return articles, nil
}

func GetVisitorList() ([]*entity.Request, error) {
	requestInfo := entity.Request{}
	requestInfos, err := requestInfo.GetListByQuery(map[string]interface{}{"is_delete": false})

	if err != nil {
		return nil, err
	}
	return requestInfos, nil
}
