package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"net"
)

type Server struct {
	listener  net.Listener
	conns     map[net.Addr]*ServerConn
	ctx       context.Context
	ctxCancel context.CancelFunc
	app       *application.Application
}

func NewServer(app *application.Application) *Server {
	return &Server{
		conns: make(map[net.Addr]*ServerConn),
		app:   app,
	}
}

func (s *Server) Run(port int, handler func(*ServerConn, []byte, int)) bool {
	listener, e := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if e != nil {
		fmt.Printf("listen error. err[%s]\n", e.Error())
		return false
	}

	s.listener = listener

	fmt.Printf("new server listening on [%d]\n", port)

	s.app.Wg.Add(1)
	go func() {
		defer s.app.Wg.Done()

		s.ctx, s.ctxCancel = context.WithCancel(s.app.Ctx)

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

			c := NewServerConn(s.app, s, conn)

			c.Handler(handler)
		}

		fmt.Println("end of listener goroutine")
	}()

	return true
}

func (s *Server) Stop() {
	s.listener.Close()
}

func (s *Server) GetFirstConn() *ServerConn {
	for _, v := range s.conns {
		return v
	}
	return nil
}
