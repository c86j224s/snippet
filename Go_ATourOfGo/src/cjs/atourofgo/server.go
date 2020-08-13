package main

import (
	"context"
	"fmt"
	"net"
)

type Server struct {
	listener  net.Listener
	conns     map[net.Addr]*ServerConn
	ctx       context.Context
	ctxCancel context.CancelFunc
}

func NewServerStart(app *Application, port int) *Server {
	s := &Server{}
	s.conns = make(map[net.Addr]*ServerConn)

	listener, e := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if e != nil {
		fmt.Printf("listen error. err[%s]\n", e.Error())
		return nil
	}

	s.listener = listener

	fmt.Printf("new server listening on [%d]\n", port)

	app.wg.Add(1)
	go func() {
		defer app.wg.Done()

		s.ctx, s.ctxCancel = context.WithCancel(app.ctx)

	AcceptLoop:
		for {
			select {
			case <-s.ctx.Done():
				break AcceptLoop
			default:
			}

			conn, e := s.listener.Accept()
			if e != nil {
				fmt.Printf("accept error. err[%s]\n", e.Error())
				//return
				break
			}

			fmt.Printf("new conn established\n")

			c := NewServerConn(app, s, conn)

			c.Handler()
		}

		fmt.Println("end of listener goroutine")
	}()

	return s
}

func (s *Server) Stop() {
	s.listener.Close()
}
