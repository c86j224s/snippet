package main

import (
	"fmt"
	"io"
	"net"
	"runtime"
	"sync"
	"time"
)

type server struct {
	listener net.Listener
	quit     chan interface{}
	wg       sync.WaitGroup
}

func newServer() *server {
	runtime.GOMAXPROCS(runtime.NumCPU() + 1)

	s := &server{}

	listener, e := net.Listen("tcp", ":8000")
	if e != nil {
		fmt.Printf("listen error")
		return nil
	}

	s.listener = listener

	fmt.Println("new server listening")

	return s
}

func (s *server) start() {
	s.wg.Add(1)
	go func() {
		defer s.wg.Done()
		for {
			conn, e := s.listener.Accept()
			if e != nil {
				select {
				case <-s.quit:
					fmt.Println("stop accept")
					return
				default:
					fmt.Println("accept error")
					fmt.Println(e)
					return
				}
			}

			s.handle(conn)
		}
	}()

	fmt.Println("new server started")
}

func (s *server) handle(conn net.Conn) {
	fmt.Printf("new conn")
	s.wg.Add(1)
	go func() {
		defer s.wg.Done()
		defer conn.Close()
		defer func() {
			fmt.Println("conn closed")
		}()

		for {
			select {
			case <-s.quit:
				fmt.Println("stop reading")
				return
			default:
			}

			conn.SetReadDeadline(time.Now().Add(1000 * time.Millisecond))

			buf := make([]byte, 1024)
			n, e := conn.Read(buf)
			if e != nil {
				if opErr, ok := e.(*net.OpError); ok && opErr.Timeout() {
					continue
				} else if e == io.EOF {
					fmt.Println("eof")
					return
				} else {
					fmt.Printf("read error ")
					fmt.Println(e)
					continue
				}
			}

			if n == 0 {
				fmt.Println("n == 0")
				return
			}

			_, e = conn.Write(buf)
			if e != nil {
				fmt.Printf("write error ")
				fmt.Println(e)
				continue
			}
		}
	}()
}

func (s *server) stop() {
	close(s.quit)
	s.listener.Close()
	s.wg.Wait()
}

func helloserver() {
	s := newServer()
	s.start()

	time.Sleep(60 * time.Second)

	s.stop()
}

func main() {
	fmt.Println("Hello, 세계")

	helloserver()
}
