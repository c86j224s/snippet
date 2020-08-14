package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"io"
	"net"
)

type ServerConn struct {
	Conn      net.Conn
	ctx       context.Context
	ctxCancel context.CancelFunc
	app       *application.Application
	srv       *Server
}

func NewServerConn(app *application.Application, srv *Server, conn net.Conn) *ServerConn {
	c := &ServerConn{
		Conn:      conn,
		ctx:       nil,
		ctxCancel: nil,
		app:       app,
		srv:       srv,
	}
	return c
}

func (c *ServerConn) Handler(handler func(*ServerConn, []byte, int)) {

	c.app.Wg.Add(1)
	go func() {
		defer func() {
			c.Conn.Close()
			c.app.Wg.Done()
		}()

		c.ctx, c.ctxCancel = context.WithCancel(c.srv.ctx)

	HandlerLoop:
		for {
			select {
			case <-c.ctx.Done():
				break HandlerLoop
			default:
			}

			buf := make([]byte, 1024)
			n, e := c.Conn.Read(buf)
			if e != nil {
				if opErr, ok := e.(*net.OpError); ok && opErr.Timeout() {
					continue
				} else if e == io.EOF {
					fmt.Println("eof")
					return
				} else {
					fmt.Printf("read error %s\n", e.Error())
					return
				}
			}

			if n == 0 {
				fmt.Println("n == 0")
				return
			}

			handler(c, buf, n)

		}

		fmt.Println("end of handler goroutine")
	}()
}
