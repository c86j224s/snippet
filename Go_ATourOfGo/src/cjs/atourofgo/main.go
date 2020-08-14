package main

import (
	"fmt"
	"time"

	"cjs/atourofgo/shared/application"
	"cjs/atourofgo/shared/network"
)

// todo : client conn manager need

func main() {
	fmt.Println("안녕, World")

	app := application.NewApplication()

	cli := network.NewClient(app)

	srv := network.NewServer(app)

	cli.Run("127.0.0.1:7000", func(c *network.ClientConn, b []byte, n int) {
		fmt.Printf("received from client conn. %s", b[:n])

		if sc := srv.GetFirstConn(); sc != nil {
			_, e := srv.GetFirstConn().Conn.Write(b[:n])
			if e != nil {
				fmt.Println("failed to proxy to client [%s], err[%s]\n", b[:n], e.Error())
				return
			}
		}
	})

	srv.Run(8000, func(c *network.ServerConn, b []byte, n int) {
		n, e := cli.Conn.Write(b[:n])
		if e != nil {
			fmt.Printf("failed to proxy to server [%s], err[%s]\n", b[:n], e.Error())
			return
		}

		/*
			_, e = c.Conn.Write(b)
			if e != nil {
				fmt.Printf("write error %s\n", e.Error())
				return
			}
		*/
	})

	time.Sleep(10 * time.Second)

	srv.Stop()
	app.Stop()

	fmt.Println("잘가, World")
}
