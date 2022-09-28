package entity

import (
	"blog-backend/logger"
	"blog-backend/model/conn"
	"blog-backend/utils/useragent"
	"time"
)

func init() {
	logger.Logger.Info("init db")
	_ = conn.MysqlConn.AutoMigrate(
		&SiteInfo{},
		&Reward{},
		&Collection{},
		&Like{},
		&Comment{},
		&Article{},
		&Category{},
		&Tags{},
		&Request{},
		&User{},
		&Link{},
		&Resource{},
	)
	_ = conn.PgConn.AutoMigrate(
		&SiteInfo{},
		&Reward{},
		&Collection{},
		&Like{},
		&Comment{},
		&Article{},
		&Category{},
		&Tags{},
		&Request{},
		&User{},
		&Link{},
		&Resource{},
	)
}

type BaseModel struct {
	ID        int       `gorm:"primary_key; index"`
	CreatedAt time.Time `json:"-"`
	UpdatedAt time.Time `json:"-"`
	IsDelete  bool      `gorm:"default:false"`
}

// SiteInfo 网站信息
type SiteInfo struct {
	BaseModel
	Author              string
	Title               string `gorm:"comment: 网站title; type:VARCHAR(255)" json:"title"`
	Keywords            string `gorm:"comment: 网站关键字; type:VARCHAR(255)" json:"keywords"`
	Description         string `gorm:"comment: 网站描述; type:VARCHAR(255)" json:"description"`
	RecordNumber        string `gorm:"comment: 备案号; type:VARCHAR(255)" json:"record_number"`
	AliPayImage         string `gorm:"comment: 支付宝收款图片; type:VARCHAR(255)" json:"alipay_image"`
	WeChatPayImage      string `gorm:"comment: 微信收款图片; type:VARCHAR(255)" json:"wechatpay_image"`
	SelfDescription     string `gorm:"comment: 个人介绍; type:text" json:"self_description"`
	SelfDescriptionHtml string `gorm:"comment: 个人介绍; type:text" json:"self_description_html"`
	Git                 string `gorm:"type:CHAR(255)" json:"git"`
	Job                 string `gorm:"type:CHAR(255)" json:"job"`
}

func (SiteInfo) TableName() string {
	return "siteinfo"
}

type Request struct {
	BaseModel
	IP         string `gorm:"not null; default:'127.0.0.1'" json:"ip"`
	Referer    string `gorm:"type: text" json:"referer"`
	URL        string `gorm:"not null" json:"url"`
	Major      int
	RemoteAddr string `gorm:"not null" json:"remote_addr"`
	UserAgent  useragent.UserAgent
}

// Tags 各种标签
type Tags struct {
	BaseModel
	Name    string `gorm:"not null; comment:tag name; unique" json:"name"`
	TagType uint   `gorm:"not null; comment:标签类型; type:VARCHAR(255)" json:"tag_type"`
}

func (Tags) TableName() string {
	return "tags"
}

// Category 分类
type Category struct {
	BaseModel
	Name        string `gorm:"not null; comment:分类名; type:VARCHAR(255)" json:"name"`
	DisplayName string `gorm:"not null; comment:分类别名; index; type:VARCHAR(255)" json:"display_name"`
	SeoDesc     string `gorm:"comment:seo描述; type:VARCHAR(255)" json:"seo_desc"`
	Support     bool   `gorm:"not null; default 1; " json:"support"`
	ParentId    uint   `gorm:"not null;default 0; comment:父类ID; index;" json:"parent_id"`
}

func (Category) TableName() string {
	return "category"
}

// Article 文章
type Article struct {
	BaseModel
	CategoryId    uint   `gorm:"index;" json:"category_id"`
	Tags          string `gorm:"type:VARCHAR(255)" json:"tags"`
	UserId        uint   `gorm:"not null; comment: 用户ID; index;" json:"user_id"`
	Title         string `gorm:"not null; comment: 标题; index; type:VARCHAR(255)" json:"title"`
	Summary       string `gorm:"not null; comment: 摘要; type:CHAR(255)" json:"summary"`
	Content       string `gorm:"not null; comment: 文章内容; type:TEXT" json:"content"`
	ClickTimes    uint   `gorm:"not null; default: 0; index;" json:"click_times"`
	LikeCount     int    `gorm:"not null; default: 0; index;" json:"like_count"`
	CollectCount  int    `gorm:"not null; default: 0; index;" json:"collect_count"`
	CommentCount  int    `gorm:"not null; default: 0; index;" json:"comment_count"`
	CanComment    bool   `gorm:"not null; default: 1;" json:"can_comment"`
	Weight        uint   `gorm:"not null;" json:"weight"`
	Support       bool   `gorm:"not null; default: true;" json:"support"`
	HeaderImgType uint   `gorm:"not null; type:int" json:"header_img_type"`
	HeaderImg     string `gorm:"type:VARCHAR(255)" json:"header_img"`
}

func (Article) TableName() string {
	return "article"
}

type Reward struct {
	BaseModel
	Who           string  `gorm:"not null; comment: 赞赏人; not null" json:"who"`
	Amount        float64 `gorm:"not null; comment: 金额; not null" json:"amount"`
	PaymentMethod uint    `gorm:"not null; comment: 支付方式; index" json:"payment_method"`
}

func (Reward) TableName() string {
	return "reward"
}

type Comment struct {
	BaseModel
	UserId    int    `gorm:"not null; comment: 用户ID; index;" json:"user_id"`
	ArticleId int    `gorm:"not null; comment: 文章ID; index; " json:"article_id"`
	Content   string `gorm:"not null; comment: 评论内容; type:TEXT" json:"content"`
	ParentId  uint   `gorm:"comment: 父评论ID; index;" json:"parent_id"`
	Ip        string
	Ua        string
	Location  string
	OS        string
}

func (Comment) TableName() string {
	return "comment"
}

type User struct {
	BaseModel     `json:"-"`
	UserName      string `gorm:"not null; unique; comment: 用户名;" json:"username"`
	Password      string `gorm:"not null;comment: 密码;" json:"-"`
	Avatar        string `gorm:"not null; comment: 头像;" json:"avatar"`
	Label         int    `gorm:"not null; comment: 标签; default: 0;" json:"label"`
	Email         string `gorm:"not null; unique; comment: 邮箱;" json:"email"`
	GitHubId      int    `gorm:"not null; comment: githubID; column:github_id" json:"github_id"`
	GitHubUrl     string `gorm:"not null; comment: github地址; column:github_url" json:"github_url"`
	IsAdmin       bool   `gorm:"default: false;" json:"is_admin"`
	ReceiveUpdate bool   `gorm:"default:true" json:"receive_update"`
	LastLogin     time.Time
}

func (User) TableName() string {
	return "user"
}

// Like 喜欢列表
type Like struct {
	BaseModel
	UserId    uint `gorm:"not null; index:user_article_id;" json:"user_id"`
	ArticleId uint `gorm:"not null; index:user_article_id;" json:"article_id"`
}

func (Like) TableName() string {
	return "like"
}

// Collection 收藏列表
type Collection struct {
	BaseModel
	UserId    uint `gorm:"not null; index:user_article_id;" json:"user_id"`
	ArticleId uint `gorm:"not null; index:user_article_id;" json:"article_id"`
}

func (Collection) TableName() string {
	return "collection"
}

type Link struct {
	BaseModel
	UserId       uint
	Title        string `gorm:"not null; comment: 标题; index; type:VARCHAR(255)" json:"title"`
	Description  string `gorm:"not null; comment: 网站描述; type:VARCHAR(255)" json:"description"`
	Email        string `gorm:"not null; unique; comment: 邮箱;" json:"email"`
	Url          string `gorm:"not null" json:"url"`
	HeaderImg    string `gorm:"type:VARCHAR(255)" json:"header_img"`
	ShowLink     bool   `gorm:"default:true" json:"show_link"`
	VerifyStatus int    `gorm:"default:0" json:"verify_status"`
}

func (Link) TableName() string {
	return "link"
}

type Resource struct {
	BaseModel
	Uuid string `gorm:"not null" json:"uuid"`
	Key  string `gorm:"not null" json:"key"`
	Type int    `gorm:"not null; comment: 资源类型" json:"type"`
}

func (Resource) TableName() string {
	return "resource"
}
