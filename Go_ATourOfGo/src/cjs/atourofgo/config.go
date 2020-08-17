package main

import (
	"cjs/atourofgo/shared/network"
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type Config struct {
	Service     network.Address   `json:"service"`
	PeerService network.Address   `json:"peer_service"`
	PeerClients []network.Address `json:"peer_clients"`
}

func ReadConfig(config_name string) *Config {
	b, e := ioutil.ReadFile(config_name)
	if e != nil {
		fmt.Printf("read file error. err=[%s]", e.Error())
		return nil
	}

	var data Config
	e = json.Unmarshal(b, &data)
	if e != nil {
		fmt.Printf("unmarshal file error. err=[%s]", e.Error())
		return nil
	}

	return &data
}
