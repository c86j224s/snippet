package main

import (
	"context"
	"fmt"
	"io"
	"net"
	"time"
)

type Server struct {
	listener  net.Listener
	conns     map[net.Addr]*Connection
	ctx       context.Context
	ctxCancel context.CancelFunc
}

type Connection struct {
	conn      net.Conn
	ctx       context.Context
	ctxCancel context.CancelFunc
}

func NewServerStart(app *Application, port int) *Server {
	s := &Server{}
	s.conns = make(map[net.Addr]*Connection)

	listener, e := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if e != nil {
		fmt.Printf("listen error %s\n", e.Error())
		return nil
	}

	s.listener = listener

	fmt.Printf("new server listening on %d\n", port)

	app.wg.Add(1)
	go func() {
		defer func() {
			s.listener.Close()
			fmt.Println("accept loop done")
			for _, conn := range s.conns {
				conn.conn.Close()
			}
			app.wg.Done()
		}()

		s.ctx, s.ctxCancel = context.WithCancel(app.ctx)

		for {
			select {
			case <-s.ctx.Done():
				s.ctxCancel()
				fmt.Println("stop accept")
				//return
				break
			default:
			}

			conn, e := s.listener.Accept()
			if e != nil {
				fmt.Printf("accept error. %s", e.Error())
				//return
				break
			}

			fmt.Println("new conn established")

			c := &Connection{}
			c.conn = conn

			s.conns[conn.LocalAddr()] = c

			app.wg.Add(1)
			go func() {
				defer func() {
					c.conn.Close()
					// 왜 출력이 안될까?
					fmt.Println("handler loop done")
					app.wg.Done()
				}()

				c.ctx, c.ctxCancel = context.WithCancel(s.ctx)

				for {
					select {
					case <-c.ctx.Done():
						// 왜 출력이 안 될까?
						c.ctxCancel()
						fmt.Printf("stop handler. %s", c.ctx.Err())
						//return
						break
					default:
					}

					c.conn.SetReadDeadline(time.Now().Add(1000 * time.Millisecond))

					buf := make([]byte, 1024)
					n, e := conn.Read(buf)
					if e != nil {
						if opErr, ok := e.(*net.OpError); ok && opErr.Timeout() {
							continue
						} else if e == io.EOF {
							fmt.Println("eof")
							//return
							break
						} else {
							fmt.Printf("read error %s\n", e.Error())
							return
						}
					}

					if n == 0 {
						fmt.Println("n == 0")
						//return
						break
					}

					_, e = conn.Write(buf)
					if e != nil {
						fmt.Printf("write error %s\n", e.Error())
						return
					}
				}
			}()
		}

		fmt.Println("end of listener goroutine")
	}()

	return s
}

func (s *Server) Stop() {
	s.listener.Close()
	s.ctxCancel()
}
