package network

import "fmt"

type Address struct {
	Addr string `json:"address"`
	Port int    `json:"port"`
}

func (a Address) ToString() string {
	return fmt.Sprintf("%s:%d", a.Addr, a.Port)
}
