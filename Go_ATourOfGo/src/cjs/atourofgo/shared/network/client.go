package network

import (
	"cjs/atourofgo/shared/application"
	"context"
)

type Client struct {
	conns  map[int]*ClientConn
	ctx    context.Context
	cancel context.CancelFunc
	app    *application.Application
}

func NewClient(app *application.Application) *Client {
	return &Client{
		conns: make(map[int]*ClientConn),
		app:   app,
	}
}

func (c *Client) Run(address string, count int, handler func(*ClientConn, []byte, int)) {
	for i := 0; i < count; i++ {
		cc := NewClientConn(c.app, c)
		c.conns[i] = cc

		cc.Run(address, handler)
	}
}

func (c *Client) GetFirstConn() *ClientConn {
	for _, v := range c.conns {
		return v
	}
	return nil
}

func (c *Client) Stop() {
	for _, v := range c.conns {
		v.Stop()
	}
}
