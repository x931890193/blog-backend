/*
config settings
*/

package config

import (
	"errors"
	"gopkg.in/yaml.v2"
	"io/ioutil"
	"path/filepath"
	"runtime"
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
	} `yaml:"Qiniu"`
	Github struct {
		ClientId     string `yaml:"client_id"`
		ClientSecret string `yaml:"client_secret"`
	} `yaml:"Github"`
	Mail struct {
		SMTPHost     string `yaml:"smtp_host"`
		SMTPPort     string `yaml:"smtp_port"`
		SMTPUsername string `yaml:"smtp_username"`
		SMTPPassword string `yaml:"smtp_password"`
		MaxClient    int    `yaml:"max_client"`
	} `yaml:"Mail"`
}

func init() {
	getCurrentFile()
	path := filepath.Join(BasePath, "config", ".config.yml")
	f, err := ioutil.ReadFile(path)
	if err != nil {
		panic(err.Error())
	}
	err = yaml.Unmarshal(f, &Cfg)
	if err != nil {
		panic(err.Error())
	}
}
