package crypt

import (
	"crypto/md5"
	"encoding/base64"
	"fmt"
)

func B64Encode(src string) string {
	sEnc := base64.StdEncoding.EncodeToString([]byte(src))
	return sEnc
}

func B64Decode(src string) (string, error) {
	decodeString, err := base64.StdEncoding.DecodeString(src)
	if err != nil {
		fmt.Printf("Error decoding string: %s ", err.Error())
		return "", err
	}
	return string(decodeString), err
}

func Md5(any []byte) (string, error) {
	srcCode := md5.Sum(any)
	// md5.Sum函数加密后返回的是字节数组，需要转换成16进制形式
	code := fmt.Sprintf("%x", srcCode)
	return code, nil
}
