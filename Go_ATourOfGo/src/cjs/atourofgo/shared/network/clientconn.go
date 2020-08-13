package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"io"
	"net"
)

type ClientConn struct {
	conn      net.Conn
	ctx       context.Context
	ctxCancel context.CancelFunc
	app       *application.Application
}

func NewClientStart(app *application.Application, address string, handler func(*ClientConn, []byte, int)) *ClientConn {
	c := &ClientConn{
		conn:      nil,
		ctx:       nil,
		ctxCancel: nil,
		app:       app,
	}

	conn, e := net.Dial("tcp", address)
	if e != nil {
		fmt.Printf("dial error. err[%s]", e.Error())
		return nil
	}

	c.conn = conn

	app.Wg.Add(1)
	go func() {
		defer app.Wg.Done()

		c.ctx, c.ctxCancel = context.WithCancel(app.Ctx)

	HandlerLoop:
		for {
			select {
			case <-c.ctx.Done():
				break HandlerLoop
			default:
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
		}
	}()

	return c
}
