package main

import (
	"fmt"
	"net/http"
	"github.com/gorilla/websocket"
)
var (
	ws = &websocket.Upgrader{
		// 允许跨域
		CheckOrigin:func(r *http.Request) bool{
			return true
		},
	}
)
func main() {
	run()
}

func run() {
	http.HandleFunc("/ws",ws1)
	http.ListenAndServe(":7777",nil)
}

func ws1(w http.ResponseWriter,r *http.Request)  {
	fmt.Println("ws1 start")
	//w.Write([]byte("ws1 hello"))
	readAndWriteWS(w,r)

}

func readAndWriteWS(w http.ResponseWriter,r *http.Request) {


	conn, err := ws.Upgrade(w, r, nil)
	if err != nil {
		fmt.Println(err)
		goto ERR
	}

	for {
		messageType, data, err := conn.ReadMessage()
		if err != nil {
			fmt.Println(err)
			goto ERR
		}
		data2 := []byte("end")
		data = append(data,data2...)
		data = append(data,'@','!')
		fmt.Printf("%s \n",data)
		conn.WriteMessage(messageType,data)
	}

ERR:
	conn.Close()
}