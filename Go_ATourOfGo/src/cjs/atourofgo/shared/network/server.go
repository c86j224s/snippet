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
}

func NewServerStart(app *application.Application, port int, handler func(*ServerConn, []byte, int)) *Server {
	s := &Server{}
	s.conns = make(map[net.Addr]*ServerConn)

	listener, e := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if e != nil {
		fmt.Printf("listen error. err[%s]\n", e.Error())
		return nil
	}

	s.listener = listener

	fmt.Printf("new server listening on [%d]\n", port)

	app.Wg.Add(1)
	go func() {
		defer app.Wg.Done()

		s.ctx, s.ctxCancel = context.WithCancel(app.Ctx)

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

			c.Handler(handler)
		}

		fmt.Println("end of listener goroutine")
	}()

	return s
}

func (s *Server) Stop() {
	s.listener.Close()
}
