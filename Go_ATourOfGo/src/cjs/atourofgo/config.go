package main

import (
	"cjs/atourofgo/shared/network"
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type PeerService struct {
	network.Address
}

type PeerClient struct {
	network.Address
}

type Config struct {
	PeerService PeerService  `json:"peer_service"`
	PeerClients []PeerClient `json:"peer_clients"`
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
