package utils

import (
	"blog-backend/logger"
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"strconv"
	"time"
)

type ReqContentType int

func (r ReqContentType) String() string {
	switch r {
	case ContentTypeJson:
		return "application/json"
	case ContentTypeProto:
		return "application/x-protobuf"
	default:
		logger.Logger.Error("to do ContentType")
		return ""
	}
}

const (
	RequestTimeOut  = 100 * time.Second
	UserAgent       = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36"
	ContentTypeJson = iota + 1
	ContentTypeProto
)

func Post(url string, data interface{}, contentType ReqContentType, extraHeaders map[string]string) ([]byte, error) {
	reqJson, _ := json.Marshal(data)
	r := bytes.NewReader(reqJson)
	req, _ := http.NewRequest("POST", url, r)
	req.Header.Add("User-Agent", UserAgent)
	req.Header.Add("Content-Type", contentType.String())
	for k, v := range extraHeaders {
		req.Header.Add(k, v)
	}
	client := http.Client{Timeout: RequestTimeOut}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			logger.Logger.Error(err.Error())
		}
	}(resp.Body)
	if resp.StatusCode != 200 {
		return nil, errors.New(strconv.Itoa(resp.StatusCode))
	}
	body, _ := ioutil.ReadAll(resp.Body)
	return body, nil
}

func Get(url string, params map[string]string, contentType ReqContentType, extraHeaders map[string]string) ([]byte, error) {
	urlParameters := url + "?"
	for k, v := range params {
		urlParameters += fmt.Sprintf("%v=%v&", k, v)
	}
	req, _ := http.NewRequest("GET", urlParameters, nil)
	req.Header.Add("User-Agent", UserAgent)
	req.Header.Add("Content-Type", contentType.String())
	for k, v := range extraHeaders {
		req.Header.Add(k, v)
	}
	client := http.Client{Timeout: RequestTimeOut}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			logger.Logger.Error(err.Error())
		}
	}(resp.Body)
	if resp.StatusCode != 200 {
		return nil, errors.New(strconv.Itoa(resp.StatusCode))
	}
	body, _ := ioutil.ReadAll(resp.Body)
	return body, nil
}
