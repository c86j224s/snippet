package main

import (
	"fmt"
	"time"

	"cjs/atourofgo/shared/application"
	"cjs/atourofgo/shared/network"
)

func main() {
	fmt.Println("안녕, World")

	app := application.NewApplication()

	srv := network.NewServerStart(app, 8000, func(c *network.ServerConn, b []byte, n int) {
		_, e := c.Conn.Write(b)
		if e != nil {
			fmt.Printf("write error %s\n", e.Error())
			return
		}
	})

	time.Sleep(10 * time.Second)

	srv.Stop()
	app.Stop()

	fmt.Println("잘가, World")
}
