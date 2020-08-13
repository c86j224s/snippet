package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Println("안녕, World")

	app := NewApplication()

	srv := NewServerStart(app.ctx, app.wg, 8000)

	time.Sleep(10 * time.Second)

	srv.ctxCancel()
	app.Stop()

	fmt.Println("잘가, World")
}
