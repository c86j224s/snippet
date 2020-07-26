package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"

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
	cli.Run(fmt.Sprintf("%s:%d", cfg.PeerClients[0].Addr, cfg.PeerClients[0].Port), 2, func(c *network.ClientConn, b []byte, n int) {
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

	sigs := make(chan os.Signal, 1)
	shutdown := make(chan bool, 1)

	signal.Notify(sigs, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		<-sigs
		shutdown <- true
	}()

	<-shutdown
	fmt.Printf("shuting down...")
	srv.Stop()
	cli.Stop()
	app.Stop()
	fmt.Printf("shutdown...")

	fmt.Println("잘가, World")
}
