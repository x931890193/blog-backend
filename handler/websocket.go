package handler

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
	"net/http"
	"time"
)

var (
	upGrader = websocket.Upgrader{
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}
	online = 0
)

func read(conn *websocket.Conn) {
	for {
		_, message, err := conn.ReadMessage()
		if nil != err {
			fmt.Println(err.Error())
			return
		}
		fmt.Println(message)
	}

}

func write(conn *websocket.Conn) {
	for {
		select {
		case <-time.After(2 * time.Second):
			if online < 0 {
				online = 0
			}
			err := conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("%v", online)))
			if err != nil {
				online--
				return
			}
		}
	}
}

func WebSocket(c *gin.Context) {
	ws, err := upGrader.Upgrade(c.Writer, c.Request, nil)
	if err != nil {
		return
	}
	online += 1
	go read(ws)
	go write(ws)
}
