package logger

import (
	"go.uber.org/zap"
	"testing"
	"time"
)

func TestLog(t *testing.T) {
	Logger.Info("test",
		zap.String("string", "string"),
		zap.Int("int", 3),
		zap.Duration("time", time.Second),
	)
	// 必须 key-value 结构形式 性能下降一点
	Logger.Sugar().Infow("test-",
		"string", "string",
		"int", 1,
		"time", time.Second,
	)
	Logger.Error("111111")
}
