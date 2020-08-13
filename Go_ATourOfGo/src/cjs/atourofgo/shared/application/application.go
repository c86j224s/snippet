package application

import (
	"context"
	"fmt"
	"sync"
)

type Application struct {
	Ctx       context.Context
	ctxCancel context.CancelFunc
	Wg        sync.WaitGroup
}

func NewApplication() *Application {
	a := &Application{}

	a.Ctx, a.ctxCancel = context.WithCancel(context.Background())

	return a
}

func (a *Application) Stop() {
	fmt.Println("application stopping")
	a.ctxCancel()
	a.Wg.Wait()
	fmt.Println("application stopped")
}
