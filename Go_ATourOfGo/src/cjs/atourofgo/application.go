package main

import (
	"context"
	"sync"
)

type Application struct {
	ctx       context.Context
	ctxCancel context.CancelFunc
	wg        sync.WaitGroup
}

func NewApplication() *Application {
	a := &Application{}

	a.ctx, a.ctxCancel = context.WithCancel(context.Background())

	return a
}

func (a *Application) Stop() {
	a.ctxCancel()
	a.wg.Wait()
}
