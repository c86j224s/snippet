package main

import (
	"encoding/json"
	"fmt"
	"github.com/fanela/goreq"
	"os"
	"strings"
)

type QbitConn struct {
	addr   string
	port   string
	client *http.Client
}

type Auth struct {
	username string
	password string
}

func (q *QbitConn) genUri(location string) string {
	return "http://" + q.addr + ":" + q.port + "/" + location
}

func (q *QbitConn) Login(auth Auth) error {
	body := "username=" + auth.username + "&password=" + auth.password
	req, err := http.NewRequest("POST", q.genUri("login"), strings.NewReader(body))
	req.Header.Add("User-Agent", "my-torrent-daemon")
	req.Header.Add("Host", "127.0.0.1")
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	resp, err := q.client.Do(req)
	if err != nil {
		return err
	}
	if resp.StatusCode != 200 {
		return fmt.Errorf("login failed. status(%s)", resp.Status)
	}
	return nil
}

func (q *QbitConn) ApiVersion() (string, error) {
	req, err := http.NewRequest("GET", q.genUri("version/api"), nil)
	resp, err := q.client.Do(req)
	if err != nil {
		return "", err
	}
	if resp.StatusCode != 200 {
		return "", fmt.Errorf("get apiversion failed. status(%s)", resp.Status)
	}
	return "2", nil
}

type Config struct {
	QbitAddr string
	QbitPort string
	QbitUser string
	QbitPass string
}

func (c *Config) Load() error {
	file, _ := os.Open("config.json")
	decoder := json.NewDecoder(file)
	err := decoder.Decode(c)
	if err != nil {
		return err
	}
	defer file.Close()
	return nil
}

func main() {
	config := Config{}
	config.Load()

	fmt.Println(config)

	jar, _ := cookiejar.New(nil)
	httpcli := http.Client{Jar: jar}
	qb := QbitConn{config.QbitAddr, config.QbitPort, &httpcli}

	err := qb.Login(Auth{config.QbitUser, config.QbitPass})
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("logged in")

	version, err := qb.ApiVersion()
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Println("version : " + version)
}
