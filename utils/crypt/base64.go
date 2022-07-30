package crypt

import (
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
