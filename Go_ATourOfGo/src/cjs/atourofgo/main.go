package main

import (
	"flag"
	"fmt"
	"time"

	"cjs/atourofgo/shared/application"
	"cjs/atourofgo/shared/network"
)

// transaction map 만들기...

func main() {
	fmt.Println("안녕, World")

	configName := flag.String("config", "./config.json", "path of config file")
	flag.Parse()

	cfg := ReadConfig(*configName)
	if cfg == nil {
		fmt.Printf("failed to open or read config.\n")
		return
	}
	if len(cfg.PeerClients) == 0 {
		fmt.Printf("peer clients length == 0\n")
		return
	}

	fmt.Println(cfg)

	app := application.NewApplication()

	cli := network.NewClient(app)

	srv := network.NewServer(app)

	// todo: 다양한 주소를 지정 가능하게...
	cli.Run(fmt.Sprintf("%s:%d", cfg.PeerClients[0].Address, cfg.PeerClients[0].Port), 2, func(c *network.ClientConn, b []byte, n int) {
		fmt.Printf("received from client conn. %s", b[:n])

		if sc := srv.GetFirstConn(); sc != nil {
			_, e := sc.Conn.Write(b[:n])
			if e != nil {
				fmt.Println("failed to proxy to client [%s], err[%s]\n", b[:n], e.Error())
				return
			}
		}
	})

	srv.Run(cfg.PeerService.Port, func(c *network.ServerConn, b []byte, n int) {
		if cc := cli.GetFirstConn(); cc != nil {
			_, e := cc.Conn.Write(b[:n])
			if e != nil {
				fmt.Printf("failed to proxy to server [%s], err[%s]\n", b[:n], e.Error())
				return
			}
		}
	})

	time.Sleep(30 * time.Second)

	srv.Stop()
	cli.Stop()
	app.Stop()

	fmt.Println("잘가, World")
}
