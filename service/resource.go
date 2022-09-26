package service

import (
	"blog-backend/cache"
	"blog-backend/model/entity"
	"blog-backend/utils"
	"blog-backend/utils/crypt"
	"encoding/json"
	"errors"
	"sort"
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

func GetPanelGroup() (interface{}, error) {
	articleObj := entity.Article{}
	articles, err := articleObj.GetAllArticle(0)
	if err != nil {
		return nil, err
	}
	user := entity.User{}
	users, err := user.GetListByQuery(map[string]interface{}{"is_delete": false})
	if err != nil {
		return nil, err
	}
	requestInfo := entity.Request{}
	requestInfos, err := requestInfo.GetListByQuery(map[string]interface{}{})
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
		if _, ok := dataMap[dateTime]; ok {
			dataMap[dateTime] += 1
		} else {
			dataMap[dateTime] = 1
		}
	}
	expectedDate := map[string]uint32{}
	actualDate := map[string]uint32{}

	thisWeek := utils.GetFirstDateOfWeek()
	lastWeek := utils.GetLastWeekFirstDate()
	for k, _ := range dataMap {
		if k >= lastWeek && k < thisWeek {
			if _, ok := expectedDate[k]; ok {
				expectedDate[k] += 1
			} else {
				expectedDate[k] = 1
			}
		} else if k >= thisWeek {
			if _, ok := actualDate[k]; ok {
				actualDate[k] += 1
			} else {
				actualDate[k] = 1
			}
		}
	}
	axisData := []uint32{}
	expectedData := []uint32{}
	actualData := []uint32{}

	sort.Slice(data, func(i, j int) bool {
		return data[i].CreatedAt.Format("2006-01-02") < data[j].CreatedAt.Format("2006-01-02")
	})

	for _, u := range data {
		k := u.CreatedAt.Format("2006-01-02")
		axisData = append(axisData, dataMap[k])
		if count, ok := actualDate[k]; ok {
			actualData = append(actualData, count)
		}
		if count, ok := expectedDate[k]; ok {
			expectedData = append(expectedData, count)
		}

	}

	return map[string][]uint32{
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
