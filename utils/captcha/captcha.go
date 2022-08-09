package captcha

import (
	"github.com/mojocn/base64Captcha"
	"math/rand"
	"time"
)

func init() {
	rand.Seed(time.Now().UnixNano())
}

type configJsonBody struct {
	Id            string
	CaptchaType   string
	VerifyValue   string
	DriverAudio   *base64Captcha.DriverAudio
	DriverString  *base64Captcha.DriverString
	DriverChinese *base64Captcha.DriverChinese
	DriverMath    *base64Captcha.DriverMath
	DriverDigit   *base64Captcha.DriverDigit
}

var store = RedisStore{}

func Generate() (id string, b64s string, err error) {
	//parse request parameters
	param := configJsonBody{
		Id:          "",
		CaptchaType: "digit",
		VerifyValue: "",
		DriverDigit: &base64Captcha.DriverDigit{
			Height:   38,
			Width:    120,
			Length:   6,
			MaxSkew:  0.5,
			DotCount: 50,
		},
	}
	var driver base64Captcha.Driver
	//create base64 encoding captcha
	switch param.CaptchaType {
	case "audio":
		driver = param.DriverAudio
	case "string":
		driver = param.DriverString.ConvertFonts()
	case "math":
		driver = param.DriverMath.ConvertFonts()
	case "chinese":
		driver = param.DriverChinese.ConvertFonts()
	default:
		driver = param.DriverDigit
	}
	c := base64Captcha.NewCaptcha(driver, store)

	id, content, answer := c.Driver.GenerateIdQuestionAnswer()
	item, err := c.Driver.DrawCaptcha(content)
	if err != nil {
		return "", "", err
	}
	err = c.Store.Set(id, answer)
	if err != nil {
		return "", "", err
	}
	b64s = item.EncodeB64string()

	return id, b64s, err
}

func Verify(id string, VerifyValue string) (res bool) {
	//parse request json body
	param := configJsonBody{
		Id:          id,
		CaptchaType: "digit",
		VerifyValue: VerifyValue,
		DriverDigit: &base64Captcha.DriverDigit{
			Height:   80,
			Width:    240,
			Length:   4,
			MaxSkew:  0.7,
			DotCount: 80,
		},
	}
	//verify the captcha
	return store.Verify(param.Id, param.VerifyValue, true)
}
