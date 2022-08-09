package captcha

import (
	"fmt"
	"testing"
)

func TestGenerateCaptcha(t *testing.T) {
	id, b64, err := Generate()
	if err != nil {
		panic(err.Error())
	}
	fmt.Println(id, b64)
}
