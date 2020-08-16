package network

import (
	"cjs/atourofgo/shared/application"
	"context"
	"fmt"
	"io"
	"net"
	"time"
)

type ClientConn struct {
	Conn    net.Conn
	ctx     context.Context
	cancel  context.CancelFunc
	hctx    context.Context
	hcancel context.CancelFunc
	app     *application.Application
	cli     *Client
}

func NewClientConn(app *application.Application, cli *Client) *ClientConn {
	return &ClientConn{
		app: app,
		cli: cli,
	}
}

func (c *ClientConn) Run(address string, handler func(*ClientConn, []byte, int)) {
	c.app.Wg.Add(1)
	go func() {
		defer c.app.Wg.Done()

		c.ctx, c.cancel = context.WithCancel(c.app.Ctx)

	ConnectLoop:
		for {
			select {
			case <-c.ctx.Done():
				break ConnectLoop
			default:
			}

			fmt.Printf("trying to connect to [%s]\n", address)

			conn, e := net.DialTimeout("tcp", address, time.Second*1)
			if e != nil {
				fmt.Printf("dial error. err[%s]\n", e.Error())
				time.Sleep(time.Second * 1)
				continue
			}

			c.Conn = conn

			c.hctx, c.hcancel = context.WithCancel(c.ctx)

			fmt.Printf("connected to [%s]. local bind [%s]\n", address, c.Conn.LocalAddr().String())

		HandlerLoop:
			for {
				select {
				case <-c.hctx.Done():
					break ConnectLoop
				default:
				}

				c.Conn.SetReadDeadline(time.Now().Add(1 * time.Second))

				buf := make([]byte, 1024)
				n, e := c.Conn.Read(buf)
				if e != nil {
					if opErr, ok := e.(*net.OpError); ok && opErr.Timeout() {
						continue
					} else if e == io.EOF {
						fmt.Println("eof")
						break HandlerLoop
					} else {
						fmt.Printf("read error %s\n", e.Error())
						break HandlerLoop
					}
				}

				if n == 0 {
					fmt.Println("n == 0")
					return
				}

				handler(c, buf, n)
			}

			fmt.Printf("disconnected to [%s]. local bind [%s]", address, c.Conn.LocalAddr().String())
		}

		fmt.Printf("end of connect goroutine.\n")
	}()
}

func (c *ClientConn) Stop() {
	if c.hcancel != nil {
		c.hcancel()
	}
	if c.cancel != nil {
		c.cancel()
	}
}
