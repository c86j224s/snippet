package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"io"
	"net"
)

// todo : ClientConn이 아니라 Client가 되어야 하고, Reconnect + multiple connections 지원이 필요하다.

type ClientConn struct {
	Conn      net.Conn
	ctx       context.Context
	ctxCancel context.CancelFunc
	app       *application.Application
}

func NewClient(app *application.Application) *ClientConn {
	return &ClientConn{
		app: app,
	}
}

func (c *ClientConn) Run(address string, handler func(*ClientConn, []byte, int)) bool {
	conn, e := net.Dial("tcp", address)
	if e != nil {
		fmt.Printf("dial error. err[%s]", e.Error())
		return false
	}

	c.Conn = conn

	c.app.Wg.Add(1)
	go func() {
		defer c.app.Wg.Done()

		c.ctx, c.ctxCancel = context.WithCancel(c.app.Ctx)

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
	}()

	return true
}
