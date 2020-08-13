package main

import (
	"context"
	"fmt"
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
	fmt.Println("application stopping")
	a.ctxCancel()
	a.wg.Wait()
	fmt.Println("application stopped")
}
