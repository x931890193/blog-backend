package conn

import (
	"blog-backend/config"
	"blog-backend/logger"
	"fmt"
	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"os"
	"time"
)

var MysqlConn *gorm.DB
var PgConn *gorm.DB

var err error

func init() {
	initMysqlDB()
	//initPgDb()
}

// initMysqlDB 初始化 MysqlDB 连接
func initMysqlDB() {
	conf := config.Cfg
	password := config.Cfg.Db.Password
	host := conf.Db.Host
	if os.Getenv("PROGRAM_ENV") == "prod" {
		password = "123456"
		host = "mysql"
	}
	dsn := fmt.Sprintf("%v:%v@tcp(%v:%v)/%v?charset=utf8mb4&parseTime=True&loc=Local",
		conf.Db.User,
		password,
		host,
		conf.Db.Port,
		conf.Db.Db)
	MysqlConn, err = gorm.Open(mysql.New(mysql.Config{
		DSN:                       dsn,   // 数据库链接配置
		SkipInitializeWithVersion: false, // 根据当前 mysql 版本自动配置
		DefaultStringSize:         256,   // string 类型字段的默认长度
		DisableDatetimePrecision:  false, // 禁用 datetime 精度， 5.6之前不支持
		DontSupportRenameIndex:    false, // 重命名索引时采用删除并新建的方式， 5.7之前不支持
		DontSupportRenameColumn:   false, // 用 change 重命名列， 8之前和 mariadb 不支持
	}), &gorm.Config{})

	if err != nil {
		logger.Logger.Error("connect to mysql database error: ", err.Error())
		panic(err.Error())
	}
	sqlDb, _ := MysqlConn.DB()
	sqlDb.SetMaxIdleConns(10)                  // 最大错误连接
	sqlDb.SetMaxOpenConns(50)                  // 最大连接数
	sqlDb.SetConnMaxLifetime(time.Second * 10) // 连接最大生命周期
	MysqlConn = MysqlConn.Debug()
}

// initPgDb 初始化 PgDB 连接
func initPgDb() {
	conf := config.Cfg
	dsn := fmt.Sprintf("host=%v user=postgres password=%v dbname=%v port=5432 sslmode=disable TimeZone=Asia/Shanghai",
		conf.Db.Host,
		conf.Db.Password,
		conf.Db.Db,
	)
	PgConn, err = gorm.Open(postgres.New(postgres.Config{DSN: dsn}), &gorm.Config{})
	if err != nil {
		panic(err.Error())
	}
}
