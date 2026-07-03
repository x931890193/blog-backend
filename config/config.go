package config

import (
	"errors"
	"gopkg.in/yaml.v2"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
)

var (
	Cfg          *config
	BasePath     string
	ArticleIdmap = map[string]int{
		"aboutme":     -1,
		"message":     -2,
		"friendslink": -3,
	}
	ArticleIdmapReverse = map[int]string{
		-1: "aboutme",
		-2: "message",
		-3: "friendslink",
	}
	UserTags = []string{"天然呆", "小萌新", "学霸", "萌萌哒", "技术宅", "忠实粉"}

	Host = "https://www.bytealien.com"
)

func substr(s string, pos, length int) string {
	runes := []rune(s)
	l := pos + length
	if l > len(runes) {
		l = len(runes)
	}
	return string(runes[pos:l])
}

func getParentDirectory(dirctory string) string {
	return substr(dirctory, 0, strings.LastIndex(dirctory, "/"))
}
func getCurrentFile() {
	_, file, _, ok := runtime.Caller(1)
	if !ok {
		panic(errors.New("Can not get current file info"))
	}
	BasePath = getParentDirectory(getParentDirectory(file))
}

type config struct {
	Server struct {
		Port string `yaml:"port"`
		Host string `yaml:"host"`
	} `yaml:"server"`
	Db struct {
		Host     string `yaml:"host"`
		Port     string `yaml:"port"`
		PgPort   string `yaml:"pg_port"`
		User     string `yaml:"user"`
		PgUser   string `yaml:"pg_user"`
		Password string `yaml:"password"`
		Db       string `yaml:"db"`
	} `yaml:"Db"`
	Cache struct {
		Host     string `yaml:"host"`
		Port     string `yaml:"port"`
		Db       int    `yaml:"db"`
		User     string `yaml:"user"`
		PassWord string `yaml:"pass_word"`
	} `yaml:"Cache"`
	Qiniu struct {
		AccessKey string `yaml:"accessKey"`
		SecretKey string `yaml:"secretKey"`
		Bucket    string `yaml:"bucket"`
		Host      string `yaml:"host"`
	} `yaml:"Qiniu"`
	GitHub struct {
		ClientId     string `yaml:"client_id"`
		ClientSecret string `yaml:"client_secret"`
		RedirectUri  string `yaml:"redirect_uri"`
	} `yaml:"GitHub"`
	AIArticle struct {
		Enabled        bool    `yaml:"enabled"`
		Spec           string  `yaml:"spec"`
		APIKey         string  `yaml:"api_key"`
		APIBase        string  `yaml:"api_base"`
		Model          string  `yaml:"model"`
		Temperature    float64 `yaml:"temperature"`
		TimeoutSeconds int     `yaml:"timeout_seconds"`
		CategoryId     uint    `yaml:"category_id"`
		UserId         uint    `yaml:"user_id"`
	} `yaml:"AIArticle"`
	Mail struct {
		SMTPHost     string `yaml:"smtp_host"`
		SMTPPort     string `yaml:"smtp_port"`
		SMTPUsername string `yaml:"smtp_username"`
		SMTPPassword string `yaml:"smtp_password"`
		MaxClient    int    `yaml:"max_client"`
	} `yaml:"Mail"`
	AliPay struct {
		PrivateKey string `yaml:"private_key"`
		PublicKey  string `yaml:"public_key"`
		AppId      string `yaml:"app_id"`
	} `yaml:"AliPay"`
	WechatPay struct {
		AppID     string `yaml:"app_iD"`
		AppSecret string `yaml:"app_secret"`
	} `yaml:"WechatPay"`
}

func init() {
	getCurrentFile()
	path := filepath.Join(BasePath, "config", ".config.yml")
	if os.Getenv("PROGRAM_ENV") == "prod" {
		path = ".config.yml"
	}
	f, err := ioutil.ReadFile(path)
	if err != nil {
		panic(err.Error())
	}
	err = yaml.Unmarshal(f, &Cfg)
	if err != nil {
		panic(err.Error())
	}
	applyAIArticleEnvOverrides()
}

func applyAIArticleEnvOverrides() {
	if value := firstEnv("AI_ARTICLE_ENABLED"); value != "" {
		if enabled, err := strconv.ParseBool(value); err == nil {
			Cfg.AIArticle.Enabled = enabled
		}
	}
	if value := firstEnv("AI_ARTICLE_SPEC"); value != "" {
		Cfg.AIArticle.Spec = value
	}
	if value := firstEnv("AI_ARTICLE_API_KEY", "OPENAI_API_KEY"); value != "" {
		Cfg.AIArticle.APIKey = value
	}
	if value := firstEnv("AI_ARTICLE_API_BASE", "OPENAI_API_BASE"); value != "" {
		Cfg.AIArticle.APIBase = value
	}
	if value := firstEnv("AI_ARTICLE_MODEL", "OPENAI_MODEL"); value != "" {
		Cfg.AIArticle.Model = value
	}
	if value := firstEnv("AI_ARTICLE_TEMPERATURE"); value != "" {
		if temperature, err := strconv.ParseFloat(value, 64); err == nil {
			Cfg.AIArticle.Temperature = temperature
		}
	}
	if value := firstEnv("AI_ARTICLE_TIMEOUT_SECONDS"); value != "" {
		if timeoutSeconds, err := strconv.Atoi(value); err == nil {
			Cfg.AIArticle.TimeoutSeconds = timeoutSeconds
		}
	}
	if value := firstEnv("AI_ARTICLE_CATEGORY_ID"); value != "" {
		if categoryID, err := strconv.ParseUint(value, 10, 32); err == nil {
			Cfg.AIArticle.CategoryId = uint(categoryID)
		}
	}
	if value := firstEnv("AI_ARTICLE_USER_ID"); value != "" {
		if userID, err := strconv.ParseUint(value, 10, 32); err == nil {
			Cfg.AIArticle.UserId = uint(userID)
		}
	}
}

func firstEnv(names ...string) string {
	for _, name := range names {
		if value := strings.TrimSpace(os.Getenv(name)); value != "" {
			return value
		}
	}
	return ""
}
