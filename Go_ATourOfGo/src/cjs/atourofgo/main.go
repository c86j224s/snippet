package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Println("안녕, World")

	app := NewApplication()

	srv := NewServerStart(app, 8000)

	time.Sleep(10 * time.Second)

	srv.Stop()
	app.Stop()

	fmt.Println("잘가, World")
}
