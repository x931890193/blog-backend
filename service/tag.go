package service

import (
	"blog-backend/model/conn"
	"blog-backend/model/entity"
	pb "blog-backend/proto"
	"encoding/json"
	"strings"
	"time"
)

const defaultTagColor = "#409EFF"

func GetTagList(pageSize, currentPage int, title, beginTime, endTime string) (*pb.ListByClassResp, int64, error) {
	tags, total, err := entity.GetTags(pageSize, currentPage, title, beginTime, endTime)
	if err != nil {
		return nil, 0, err
	}
	counts, err := ArticleTagCounts(0)
	if err != nil {
		return nil, 0, err
	}
	resp := &pb.ListByClassResp{Tags: []*pb.Tags{}}
	for _, tag := range tags {
		resp.Tags = append(resp.Tags, &pb.Tags{
			XId:        uint32(tag.ID),
			Name:       tag.Name,
			Color:      normalizeTagColor(tag.Color),
			Count:      uint32(counts[tag.Name]),
			CreateTime: formatAdminTime(tag.CreatedAt),
		})
	}
	return resp, total, nil
}

func AddTag(name, color string) error {
	return entity.CreateTag(strings.TrimSpace(name), normalizeTagColor(color))
}

func UpdateTag(id int, name, color string) error {
	name = strings.TrimSpace(name)
	color = normalizeTagColor(color)
	oldTag, err := entity.GetTagByID(id)
	if err != nil {
		return err
	}
	if err := entity.UpdateTag(id, name, color); err != nil {
		return err
	}
	if oldTag.Name != name {
		return RenameArticleTag(oldTag.Name, name)
	}
	return nil
}

func DeleteTags(ids []int) error {
	return entity.DeleteTagsByIds(ids)
}

func normalizeTagColor(color string) string {
	color = strings.TrimSpace(color)
	if color == "" {
		return defaultTagColor
	}
	return color
}

func ArticleTagCounts(categoryID uint) (map[string]int, error) {
	article := entity.Article{}
	articles, err := article.GetAllArticle(categoryID)
	if err != nil {
		return nil, err
	}
	counts := map[string]int{}
	for _, item := range articles {
		for _, tag := range parseArticleTags(item.Tags) {
			counts[tag]++
		}
	}
	return counts, nil
}

func parseArticleTags(raw string) []string {
	raw = strings.TrimSpace(raw)
	if raw == "" {
		return nil
	}
	var tags []string
	if err := json.Unmarshal([]byte(raw), &tags); err == nil {
		return normalizeArticleTags(tags)
	}
	raw = strings.NewReplacer(",", " ", "，", " ", "、", " ", "|", " ").Replace(raw)
	return normalizeArticleTags(strings.Fields(raw))
}

func normalizeArticleTags(tags []string) []string {
	res := make([]string, 0, len(tags))
	seen := map[string]bool{}
	for _, tag := range tags {
		tag = strings.TrimSpace(tag)
		if tag == "" || seen[tag] {
			continue
		}
		seen[tag] = true
		res = append(res, tag)
	}
	return res
}

func RenameArticleTag(oldName, newName string) error {
	oldName = strings.TrimSpace(oldName)
	newName = strings.TrimSpace(newName)
	if oldName == "" || newName == "" || oldName == newName {
		return nil
	}
	var articles []*entity.Article
	if err := conn.MysqlConn.Model(&entity.Article{}).Where("tags LIKE ?", "%"+oldName+"%").Find(&articles).Error; err != nil {
		return err
	}
	now := time.Now()
	for _, article := range articles {
		tags := parseArticleTags(article.Tags)
		changed := false
		for i, tag := range tags {
			if tag == oldName {
				tags[i] = newName
				changed = true
			}
		}
		if !changed {
			continue
		}
		marshal, _ := json.Marshal(normalizeArticleTags(tags))
		if err := conn.MysqlConn.Model(&entity.Article{}).Where("id = ?", article.ID).Updates(map[string]interface{}{
			"tags":       string(marshal),
			"updated_at": now,
		}).Error; err != nil {
			return err
		}
	}
	return nil
}
