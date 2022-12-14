package mail

import (
	"blog-backend/config"
	"blog-backend/logger"
	"blog-backend/utils/struct_util"
	"fmt"
	"github.com/jordan-wright/email"
	"net/smtp"
	"os"
	"reflect"
	"sync"
	"time"
)

var (
	err          error
	pool         *email.Pool
	commentTpl   = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>有一条评论@了你</title></head><body><div><includetail><div align=\"center\"><div class=\"open_email\" style=\"margin-left: 8px; margin-top: 8px; margin-bottom: 8px; margin-right: 8px;\"><div><br><span class=\"genEmailContent\"><div id=\"cTMail-Wrap\" style=\"word-break: break-all;box-sizing:border-box;text-align:center;min-width:320px; max-width:660px; border:1px solid #f6f6f6; background-color:#f7f8fa; margin:auto; padding:20px 0 30px; font-family:'helvetica neue',PingFangSC-Light,arial,'hiragino sans gb','microsoft yahei ui','microsoft yahei',simsun,sans-serif\"><div class=\"main-content\" style=\"\"><table style=\"width:100%;font-weight:300;margin-bottom:10px;border-collapse:collapse\"><tbody><tr style=\"font-weight:300\"><td style=\"width:3%;max-width:30px;\"></td><td style=\"max-width:600px;\"><div id=\"cTMail-logo\" style=\"width:92px; height:25px;\"><a href=\"\"><img border=\"0\" src=\"https://cdn.mongona.com/upload/image/site/favicon_ZuliWuW.ico\" style=\"width:36px; height:36px;display:block\"></a></div><p style=\"height:2px;background-color: #00a4ff;border: 0;font-size:0;padding:0;width:100%;margin-top:20px;\"></p><div id=\"cTMail-inner\" style=\"background-color:#fff; padding:23px 0 20px;box-shadow: 0 1px 1px 0 rgba(122, 55, 55, 0.2);text-align:left;\"><table style=\"width:100%;font-weight:300;margin-bottom:10px;border-collapse:collapse;text-align:left;\"><tbody><tr style=\"font-weight:300\"><td style=\"width:3.2%;max-width:30px;\"></td><td style=\"max-width:480px;text-align:left;\"><h1 id=\"cTMail-title\" style=\"font-size: 20px; line-height: 50px; margin: 0 0 22px;\"><strong>您有一条新评论提醒～ </strong></h1><p id=\"cTMail-userName\" style=\"font-size:14px;color:#333; line-height:30px; margin:0;\">Halo&nbsp;<strong>${username}</strong>～:</p><p id=\"cTMail-content\" style=\"font-size:14px;color:#333; line-height:30px; margin:0;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;您好！</p><p class=\"cTMail-content\" style=\"line-height: 24px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all;\"><span style=\"color: rgb(51, 51, 51); font-size: 14px;\"><strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;${who}</strong> 在您 <strong>${time}</strong> 于 <strong>${site}</strong> 发表的评论【<strong>${origin_comment}</strong>】中回复说到: 【<strong style=\"color: #67C23A\">${new_comment}</strong>】</span></p><p class=\"cTMail-content\" style=\"line-height: 50px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all;\"><span style=\"color: rgb(51, 51, 51); font-size: 14px;\">查看详情，可点击下方链接。<span style=\"font-weight: bold;\">如有打扰，请您谅解。</span></span></p><p class=\"cTMail-content\" style=\"font-size: 14px; color: rgb(51, 51, 51); line-height: 24px; margin: 6px 0 0; word-wrap: break-word; word-break: break-all;\"><a id=\"cTMail-btn\" href=\"${url}\" title=\"点击此处查看详情\" style=\"font-size: 16px; line-height: 45px; display: block; background-color: rgb(0, 164, 255); color: rgb(255, 255, 255); text-align: center; text-decoration: none; margin-top: 20px; border-radius: 3px;\">点击此处查看详情</a></p><p class=\"cTMail-content\" style=\"line-height: 24px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all;\"><span style=\"color: rgb(51, 51, 51); font-size: 14px;\"><br>无法正常显示？请复制以下链接至浏览器打开：<br><a href=\"${url}\" title=\"\" style=\"color: rgb(0, 164, 255); text-decoration: none; word-break: break-all; overflow-wrap: normal; font-size: 14px;\">详情链接</a></span></p></td><td style=\"width:3.2%;max-width:30px;\"></td></tr></tbody></table></div><div id=\"cTMail-copy\" style=\"text-align:center; font-size:12px; line-height:18px; color:#999\"><table style=\"width:100%;font-weight:300;margin-bottom:10px;border-collapse:collapse\"><tbody><tr style=\"font-weight:300\"><td style=\"width:3.2%;max-width:30px;\"></td><td style=\"max-width:540px;\"><p style=\"text-align:center; margin:20px auto 14px auto;font-size:12px;color:#999;\">此邮件由自动发出，请勿回复。</p><p id=\"cTMail-rights\" style=\"max-width: 100%; margin:auto;font-size:12px;color:#999;text-align:center;line-height:22px;\"><img border=\"0\" src=\"https://cdn.mongona.com/bytealien/qrcode_www.bytealien.com.png\" style=\"width:84px; height:84px; margin:0 auto;\"><br>了解更多Python,Django,go,Java,sql,linux,javascript,jquery,git,教程,软件,编程,开发,运维,云计算,网络,互联网<br></p></td><td style=\"width:3.2%;max-width:30px;\"></td></tr></tbody></table></div></td><td style=\"width:3%;max-width:30px;\"></td></tr></tbody></table></div></div></span></div></div></div></includetail></div></body></html>"
	newUpdateTpl = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>新文章阅读提醒@了你</title></head><body><div><includetail><div align=\"center\"><div class=\"open_email\" style=\"margin-left: 8px; margin-top: 8px; margin-bottom: 8px; margin-right: 8px;\"><div><br><span class=\"genEmailContent\"><div id=\"cTMail-Wrap\" style=\"word-break: break-all;box-sizing:border-box;text-align:center;min-width:320px; max-width:660px; border:1px solid #f6f6f6; background-color:#f7f8fa; margin:auto; padding:20px 0 30px; font-family:'helvetica neue',PingFangSC-Light,arial,'hiragino sans gb','microsoft yahei ui','microsoft yahei',simsun,sans-serif\"><div class=\"main-content\" style=\"\"><table style=\"width:100%;font-weight:300;margin-bottom:10px;border-collapse:collapse\"><tbody><tr style=\"font-weight:300\"><td style=\"width:3%;max-width:30px;\"></td><td style=\"max-width:600px;\"><div id=\"cTMail-logo\" style=\"width:92px; height:25px;\"><a href=\"\"><img border=\"0\" src=\"https://cdn.mongona.com/upload/image/site/favicon_ZuliWuW.ico\" style=\"width:36px; height:36px;display:block\"></a></div><p style=\"height:2px;background-color: #00a4ff;border: 0;font-size:0;padding:0;width:100%;margin-top:20px;\"></p><div id=\"cTMail-inner\" style=\"background-color:#fff; padding:23px 0 20px;box-shadow: 0 1px 1px 0 rgba(122, 55, 55, 0.2);text-align:left;\"><table style=\"width:100%;font-weight:300;margin-bottom:10px;border-collapse:collapse;text-align:left;\"><tbody><tr style=\"font-weight:300\"><td style=\"width:3.2%;max-width:30px;\"></td><td style=\"max-width:480px;text-align:left;\"><h1 id=\"cTMail-title\" style=\"font-size: 20px; line-height: 50px; margin: 0 0 22px;\"><strong>新文章更新提醒</strong></h1><p id=\"cTMail-userName\" style=\"font-size:14px;color:#333; line-height:30px; margin:0;\">Halo&nbsp;<strong>${username}</strong>～:</p><p id=\"cTMail-content\" style=\"font-size:14px;color:#333; line-height:30px; margin:0;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;您好！</p><p class=\"cTMail-content\" style=\"line-height: 24px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all;\"><span style=\"color: rgb(51, 51, 51); font-size: 14px;\"><strong>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;站长</strong> 在 <strong>${site}</strong>于 <strong>${time}</strong> 发表了新文章:<strong>【${title}】</strong> </span></p><p class=\"cTMail-content\" style=\"line-height: 24px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all; text-align: center\"><span style=\"color: rgb(51, 51, 51); font-size: 18px;\">${summary}</span></p><p class=\"cTMail-content\" style=\"line-height: 50px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all;\"><span style=\"color: rgb(51, 51, 51); font-size: 14px;\">查看全文详情，可点击下方链接。<span style=\"font-weight: bold;\">如有打扰，请您谅解。</span></span></p><p class=\"cTMail-content\" style=\"font-size: 14px; color: rgb(51, 51, 51); line-height: 24px; margin: 6px 0 0; word-wrap: break-word; word-break: break-all;\"><a id=\"cTMail-btn\" href=\"${url}\" title=\"点击此处查看全文详情\" style=\"font-size: 16px; line-height: 45px; display: block; background-color: rgb(0, 164, 255); color: rgb(255, 255, 255); text-align: center; text-decoration: none; margin-top: 20px; border-radius: 3px;\">点击此处查看全文详情</a></p><p class=\"cTMail-content\" style=\"line-height: 24px; margin: 6px 0 0; overflow-wrap: break-word; word-break: break-all;\"><span style=\"color: rgb(51, 51, 51); font-size: 14px;\"><br>无法正常显示？请复制以下链接至浏览器打开：<br><a href=\"${url}\" title=\"\" style=\"color: rgb(0, 164, 255); text-decoration: none; word-break: break-all; overflow-wrap: normal; font-size: 14px;\">详情链接</a></span></p></td><td style=\"width:3.2%;max-width:30px;\"></td></tr></tbody></table></div><div id=\"cTMail-copy\" style=\"text-align:center; font-size:12px; line-height:18px; color:#999\"><table style=\"width:100%;font-weight:300;margin-bottom:10px;border-collapse:collapse\"><tbody><tr style=\"font-weight:300\"><td style=\"width:3.2%;max-width:30px;\"></td><td style=\"max-width:540px;\"><p style=\"text-align:center; margin:20px auto 14px auto;font-size:12px;color:#999;\">此邮件由自动发出，请勿回复。</p><p id=\"cTMail-rights\" style=\"max-width: 100%; margin:auto;font-size:12px;color:#999;text-align:center;line-height:22px;\"><img border=\"0\" src=\"https://cdn.mongona.com/bytealien/qrcode_www.bytealien.com.png\" style=\"width:84px; height:84px; margin:0 auto;\"><br>了解更多Python,Django,go,Java,sql,linux,javascript,jquery,git,教程,软件,编程,开发,运维,云计算,网络,互联网<br></p></td><td style=\"width:3.2%;max-width:30px;\"></td></tr></tbody></table></div></td><td style=\"width:3%;max-width:30px;\"></td></tr></tbody></table></div></div></span></div></div></div></includetail></div></body></html>"
)

type Comment struct {
	Site          string `json:"site"`
	Username      string `json:"username"`
	Who           string `json:"who"`
	Time          string `json:"time"`
	OriginComment string `json:"origin_comment"`
	NewComment    string `json:"new_comment"`
	Url           string `json:"url"`
}

type NewUpdate struct {
	Username string `json:"username"`
	Site     string `json:"site"`
	Time     string `json:"time"`
	Title    string `json:"title"`
	Summary  string `json:"summary"`
	Url      string `json:"url"`
}

func init() {
	pool, err = email.NewPool(fmt.Sprintf("%v:%v", config.Cfg.Mail.SMTPHost, config.Cfg.Mail.SMTPPort), config.Cfg.Mail.MaxClient, smtp.PlainAuth("", config.Cfg.Mail.SMTPUsername, config.Cfg.Mail.SMTPPassword, config.Cfg.Mail.SMTPHost))
	if err != nil {
		logger.Logger.Fatal(err.Error())
	}
}

func SendEmail(receivers []string, subject string, structTpl interface{}, wg *sync.WaitGroup) {
	var body string
	defer func() {
		if wg != nil {
			wg.Done()
		}
	}()
	bodyMap, err := struct_util.StructToMap(structTpl, "json", "StructToMap")
	if err != nil {
		logger.Logger.Error(err.Error())
		return
	}
	switch reflect.TypeOf(structTpl) {

	case reflect.TypeOf(Comment{}):
		body = os.Expand(commentTpl, func(k string) string { return bodyMap[k].(string) })
	case reflect.TypeOf(NewUpdate{}):
		body = os.Expand(newUpdateTpl, func(k string) string { return bodyMap[k].(string) })
	default:
		return
	}
	e := &email.Email{
		From:    fmt.Sprintf("bytealien<%s>", config.Cfg.Mail.SMTPUsername),
		To:      receivers,
		Subject: subject,
		HTML:    []byte(body),
	}
	err = pool.Send(e, 100*time.Second)
	if err != nil {
		logger.Logger.Fatal(err.Error())
	}
}
