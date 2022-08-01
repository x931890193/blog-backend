package handler

import "fmt"

type ErrorCode uint32

const (
	AuthError ErrorCode = iota + 1
	ParamsError
	DbError
	LogicError
	UnKnown
)

var ErrorMsg = map[ErrorCode]string{
	AuthError:   "认证失败",
	ParamsError: "参数错误",
	DbError:     "数据库异常",
	LogicError:  "逻辑错误",
	UnKnown:     "未知错误",
}

func ConvertMsg(code ErrorCode, extraMsg interface{}) string {
	if msg, ok := ErrorMsg[code]; ok {
		return fmt.Sprintf("%v : %v", msg, extraMsg)
	}
	return fmt.Sprintf("%v : %v", "未知错误", extraMsg)
}
