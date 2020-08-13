package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"io"
	"net"
)

type ServerConn struct {
	conn      net.Conn
	ctx       context.Context
	ctxCancel context.CancelFunc
	app       *application.Application
	srv       *Server
}

func NewServerConn(app *application.Application, srv *Server, conn net.Conn) *ServerConn {
	c := &ServerConn{
		conn:      conn,
		ctx:       nil,
		ctxCancel: nil,
		app:       app,
		srv:       srv,
	}
	return c
}

func (c *ServerConn) Handler(handler func(*ServerConn, []byte, int)) {
	c.srv.conns[c.conn.LocalAddr()] = c

	c.app.Wg.Add(1)
	go func() {
		defer func() {
			c.conn.Close()
			c.app.Wg.Done()
		}()

		c.ctx, c.ctxCancel = context.WithCancel(c.srv.ctx)

	HandlerLoop:
		for {
			select {
			case <-c.ctx.Done():
				break HandlerLoop
			}

			buf := make([]byte, 1024)
			n, e := c.conn.Read(buf)
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

			_, e = c.conn.Write(buf)
			if e != nil {
				fmt.Printf("write error %s\n", e.Error())
				return
			}
		}

		fmt.Println("end of handler goroutine")
	}()
}
