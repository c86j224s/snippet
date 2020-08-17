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

	srv := network.NewServer(app)

	pcli := network.NewClient(app)

	psrv := network.NewServer(app)

	pcli.Run(cfg.PeerClients, func(c *network.ClientConn, b []byte, n int) {
		fmt.Printf("received from peer client conn. %s", b[:n])
	})

	psrv.Run(cfg.PeerService, func(c *network.ServerConn, b []byte, n int) {
		fmt.Printf("received from peer server conn. %s", b[:n])
	})

	srv.Run(cfg.Service, func(c *network.ServerConn, b []byte, n int) {
		fmt.Printf("received from service server conn. [%s]", b[:n])

		pcli.Broadcast(b, n)
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
	psrv.Stop()
	pcli.Stop()
	srv.Stop()
	app.Stop()
	fmt.Printf("shutdown...")

	fmt.Println("잘가, World")
}
