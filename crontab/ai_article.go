package crontab

import (
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/model/entity"
	"blog-backend/service"
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"strings"
	"time"
)

const aiArticleMarkerPrefix = "ai-generated-article:"

type aiArticleTopic struct {
	Title string
	Tags  []string
	Brief string
}

type aiArticleGenerated struct {
	Title   string `json:"title"`
	Summary string `json:"summary"`
	Tags    string `json:"tags"`
	Content string `json:"content"`
}

type chatCompletionResp struct {
	Choices []struct {
		Message struct {
			Content string `json:"content"`
		} `json:"message"`
	} `json:"choices"`
}

var aiArticleTopics = []aiArticleTopic{
	{
		Title: "AI 工程化：从模型调用到可观测系统",
		Tags:  []string{"AI", "工程化", "可观测性"},
		Brief: "围绕 RAG、工具调用、日志、评测、成本控制和上线稳定性写一篇实践型文章。",
	},
	{
		Title: "个人博客如何长期维护：内容、工具和自动化",
		Tags:  []string{"个人站", "自动化", "产品"},
		Brief: "面向个人开发者，讨论内容生产、后台工具、SEO、AI 助手和轻量商业化。",
	},
	{
		Title: "Go 后端服务的稳定性清单",
		Tags:  []string{"Go", "后端", "稳定性"},
		Brief: "覆盖配置、日志、数据库、缓存、定时任务、部署、回滚和监控。",
	},
	{
		Title: "金融科技系统中的实时风控设计",
		Tags:  []string{"金融科技", "风控", "实时系统"},
		Brief: "讨论规则、特征、模型、人工复核、灰度策略和审计链路如何协同。",
	},
	{
		Title: "能源数字化项目里的数据平台建设",
		Tags:  []string{"能源科技", "数据平台", "自动化"},
		Brief: "从设备数据、市场数据、预测模型、调度策略和业务看板展开。",
	},
}

func GenerateAIArticle() {
	if config.Cfg == nil || !config.Cfg.AIArticle.Enabled {
		return
	}
	if strings.TrimSpace(config.Cfg.AIArticle.APIKey) == "" {
		logger.Logger.Warn("AIArticle enabled but api_key is empty")
		return
	}
	topic := pickAIArticleTopic(time.Now())
	generated, marker, err := buildGeneratedAIArticle(topic)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("generate AI article failed: %s", err.Error()))
		return
	}
	exists, err := entity.ArticleContentExists(marker)
	if err != nil {
		logger.Logger.Error(fmt.Sprintf("check AI article marker failed: %s", err.Error()))
		return
	}
	if exists {
		logger.Logger.Info(fmt.Sprintf("AI article already exists: %s", marker))
		return
	}
	if err := createAIArticle(topic, generated, marker); err != nil {
		logger.Logger.Error(fmt.Sprintf("save AI article failed: %s", err.Error()))
		return
	}
	logger.Logger.Info(fmt.Sprintf("AI article generated: %s", generated.Title))
}

func pickAIArticleTopic(now time.Time) aiArticleTopic {
	if len(aiArticleTopics) == 0 {
		return aiArticleTopic{Title: "AI 工程实践", Tags: []string{"AI", "工程化"}}
	}
	return aiArticleTopics[now.YearDay()%len(aiArticleTopics)]
}

func buildGeneratedAIArticle(topic aiArticleTopic) (*aiArticleGenerated, string, error) {
	uid := aiArticleUID(topic, time.Now())
	marker := aiArticleMarkerPrefix + uid
	content, err := callAIArticleAPI(buildAIArticlePrompt(topic))
	if err != nil {
		return nil, marker, err
	}
	generated, err := parseGeneratedAIArticle(content)
	if err != nil {
		return nil, marker, err
	}
	if strings.TrimSpace(generated.Title) == "" {
		generated.Title = topic.Title
	}
	if strings.TrimSpace(generated.Summary) == "" {
		generated.Summary = truncateRunes(topic.Brief, 180)
	}
	if strings.TrimSpace(generated.Content) == "" {
		return nil, marker, errors.New("AI article content is empty")
	}
	return generated, marker, nil
}

func aiArticleUID(topic aiArticleTopic, now time.Time) string {
	hash := sha256.Sum256([]byte(now.Format("2006-01-02") + ":" + topic.Title))
	return hex.EncodeToString(hash[:])[:24]
}

func buildAIArticlePrompt(topic aiArticleTopic) string {
	return fmt.Sprintf(`请写一篇中文原创技术博客文章，主题如下：
标题方向：%s
关键词：%s
写作说明：%s

要求：
1. 文章面向个人开发者和小团队，强调工程实践和可落地建议。
2. 正文 900-1400 中文字，结构清晰，有 3-5 个 Markdown 二级标题。
3. 不要编造实时新闻、价格、政策或具体公司数据。
4. 输出必须是 JSON，不要 Markdown 代码块，格式：
{"title":"...","summary":"80-140字摘要","tags":"AI 工程化 产品","content":"正文，允许 Markdown 小标题和列表"}`, topic.Title, strings.Join(topic.Tags, " "), topic.Brief)
}

func callAIArticleAPI(prompt string) (string, error) {
	apiBase := strings.TrimRight(config.Cfg.AIArticle.APIBase, "/")
	if apiBase == "" {
		apiBase = "https://api.openai.com/v1"
	}
	model := strings.TrimSpace(config.Cfg.AIArticle.Model)
	if model == "" {
		model = "gpt-4o-mini"
	}
	temperature := config.Cfg.AIArticle.Temperature
	if temperature <= 0 {
		temperature = 0.7
	}
	timeout := config.Cfg.AIArticle.TimeoutSeconds
	if timeout <= 0 {
		timeout = 90
	}
	payload := map[string]interface{}{
		"model": model,
		"messages": []map[string]string{
			{"role": "system", "content": "你是资深中文技术博客作者，擅长写有观点、有结构、有实践价值的工程文章。"},
			{"role": "user", "content": prompt},
		},
		"temperature": temperature,
	}
	body, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}
	req, err := http.NewRequest(http.MethodPost, apiBase+"/chat/completions", bytes.NewReader(body))
	if err != nil {
		return "", err
	}
	req.Header.Set("Authorization", "Bearer "+config.Cfg.AIArticle.APIKey)
	req.Header.Set("Content-Type", "application/json")
	client := &http.Client{Timeout: time.Duration(timeout) * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	respBody, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}
	if resp.StatusCode >= http.StatusBadRequest {
		return "", fmt.Errorf("AI API HTTP %d: %s", resp.StatusCode, truncateRunes(string(respBody), 500))
	}
	chatResp := chatCompletionResp{}
	if err := json.Unmarshal(respBody, &chatResp); err != nil {
		return "", err
	}
	if len(chatResp.Choices) == 0 || strings.TrimSpace(chatResp.Choices[0].Message.Content) == "" {
		return "", errors.New("AI API returned empty choices")
	}
	return chatResp.Choices[0].Message.Content, nil
}

func parseGeneratedAIArticle(value string) (*aiArticleGenerated, error) {
	value = strings.TrimSpace(value)
	value = strings.TrimPrefix(value, "```json")
	value = strings.TrimPrefix(value, "```")
	value = strings.TrimSuffix(value, "```")
	value = strings.TrimSpace(value)
	generated := &aiArticleGenerated{}
	if err := json.Unmarshal([]byte(value), generated); err == nil {
		return generated, nil
	}
	start := strings.Index(value, "{")
	end := strings.LastIndex(value, "}")
	if start < 0 || end <= start {
		return nil, errors.New("AI response is not JSON")
	}
	if err := json.Unmarshal([]byte(value[start:end+1]), generated); err != nil {
		return nil, err
	}
	return generated, nil
}

func createAIArticle(topic aiArticleTopic, generated *aiArticleGenerated, marker string) error {
	tags := tagsForAIArticle(topic, generated.Tags)
	tagsJSON, _ := json.Marshal(tags)
	categoryID := config.Cfg.AIArticle.CategoryId
	if categoryID == 0 {
		categoryID = 2
	}
	userID := config.Cfg.AIArticle.UserId
	if userID == 0 {
		userID = 1
	}
	article := entity.Article{
		CategoryId:    categoryID,
		Tags:          string(tagsJSON),
		UserId:        userID,
		Title:         truncateRunes(generated.Title, 180),
		Summary:       truncateRunes(generated.Summary, 240),
		Content:       buildAIArticleContent(marker, generated.Content),
		ClickTimes:    0,
		CanComment:    true,
		Weight:        60,
		Support:       true,
		HeaderImgType: 0,
		HeaderImg:     "",
	}
	if err := article.CreateOne(); err != nil {
		return err
	}
	service.InvalidateArticleTotalCount()
	return nil
}

func tagsForAIArticle(topic aiArticleTopic, generatedTags string) []string {
	res := append([]string{}, topic.Tags...)
	generatedTags = strings.NewReplacer(",", " ", "，", " ", "、", " ", "|", " ").Replace(generatedTags)
	for _, tag := range strings.Fields(generatedTags) {
		if tag != "" {
			res = append(res, tag)
		}
	}
	seen := map[string]bool{}
	dedup := []string{}
	for _, tag := range res {
		if seen[tag] {
			continue
		}
		seen[tag] = true
		dedup = append(dedup, tag)
		if len(dedup) >= 6 {
			break
		}
	}
	return dedup
}

func buildAIArticleContent(marker, content string) string {
	return fmt.Sprintf(`<!-- %s -->

> 本文由站点定时任务调用 AI API 辅助生成，主题和内容会持续迭代；发布后建议人工抽查事实和表达。

%s
`, marker, strings.TrimSpace(content))
}

func truncateRunes(value string, max int) string {
	value = strings.TrimSpace(value)
	if max <= 0 {
		return value
	}
	runes := []rune(value)
	if len(runes) <= max {
		return value
	}
	return string(runes[:max])
}
