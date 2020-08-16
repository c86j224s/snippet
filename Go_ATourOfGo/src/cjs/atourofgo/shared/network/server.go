package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"net"
	"sync/atomic"
)

type Server struct {
	listener  net.Listener
	conns     map[int32]*ServerConn
	ctx       context.Context
	ctxCancel context.CancelFunc
	app       *application.Application
}

func NewServer(app *application.Application) *Server {
	return &Server{
		conns: make(map[int32]*ServerConn),
		app:   app,
	}
}

func (s *Server) Run(addr Address, handler func(*ServerConn, []byte, int)) bool {
	listener, e := net.Listen("tcp", addr.ToString())
	if e != nil {
		fmt.Printf("listen error. err[%s]\n", e.Error())
		return false
	}

	s.listener = listener

	fmt.Printf("new server listening on [%d]\n", addr.Port)

	s.app.Wg.Add(1)
	go func() {
		defer s.app.Wg.Done()

		s.ctx, s.ctxCancel = context.WithCancel(s.app.Ctx)
		var sid int32 = 0

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
				break AcceptLoop
			}

			fmt.Printf("new conn established\n")

			c := NewServerConn(s.app, s, conn)
			s.conns[atomic.AddInt32(&sid, 1)] = c

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

func (s *Server) GetConn(idx int32) *ServerConn {
	return s.conns[idx]
}

// todo: round robin connection getter 만들기
