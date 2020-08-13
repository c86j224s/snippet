package main

import (
	"context"
	"fmt"
	"io"
	"net"
	"sync"
)

type Server struct {
	listener  net.Listener
	ctx       context.Context
	ctxCancel context.CancelFunc
}

type Connection struct {
	conn      net.Conn
	ctx       context.Context
	ctxCancel context.CancelFunc
}

func NewServerStart(ctx context.Context, wg sync.WaitGroup, port int) *Server {
	s := &Server{}

	listener, e := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if e != nil {
		fmt.Printf("listen error %s\n", e.Error())
		return nil
	}

	s.listener = listener

	fmt.Printf("new server listening on %d\n", port)

	wg.Add(1)
	go func() {
		defer func() {
			s.listener.Close()
			fmt.Println("accept loop done")
			wg.Done()
		}()

		s.ctx, s.ctxCancel = context.WithCancel(ctx)

		for {
			select {
			case <-s.ctx.Done():
				fmt.Println("stop accept")
				return
			default:
			}

			conn, e := s.listener.Accept()
			if e != nil {
				fmt.Printf("accept error. %s", e.Error())
				return
			}

			fmt.Println("new conn established")

			c := &Connection{}
			c.conn = conn

			wg.Add(1)
			go func() {
				defer func() {
					c.conn.Close()
					// 왜 출력이 안될까?
					fmt.Println("handler loop done")
					wg.Done()
				}()

				c.ctx, c.ctxCancel = context.WithCancel(s.ctx)

				for {
					select {
					case <-c.ctx.Done():
						// 왜 출력이 안 될까?
						fmt.Printf("stop handler. %s", c.ctx.Err())
						return
					default:
					}

					//c.conn.SetReadDeadline(time.Now().Add(1000 * time.Millisecond))

					buf := make([]byte, 1024)
					n, e := conn.Read(buf)
					if e != nil {
						if opErr, ok := e.(*net.OpError); ok && opErr.Timeout() {
							continue
						} else if e == io.EOF {
							fmt.Println("eof")
							return
						} else {
							fmt.Printf("read error %s\n", e.Error())
							continue
						}
					}

					if n == 0 {
						fmt.Println("n == 0")
						return
					}

					_, e = conn.Write(buf)
					if e != nil {
						fmt.Printf("write error %s\n", e.Error())
						continue
					}
				}
			}()
		}

	}()

	return s
}
