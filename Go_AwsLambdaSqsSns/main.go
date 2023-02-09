package main

import (
	"alss/sqslambda"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"sync"
	"time"
)

type ExampleContext struct {
}

type ExampleMessage struct {
	Name        string
	ExampleType string
}

func ExampleHandler(ctx context.Context, fn *sqslambda.Function[ExampleContext], body string) (err error) {
	defer func() {
		if r := recover(); r != nil {
			if asErr, ok := r.(error); ok {
				err = asErr
			} else {
				err = fmt.Errorf("panic in example handler, r[%+v]", r)
			}
		}
	}()

	msg := ExampleMessage{}
	if err := json.Unmarshal([]byte(body), &msg); err != nil {
		return err
	}

	wg := sync.WaitGroup{}

	// ...
	// do something
	sleepChan := make(chan error, 1)
	wg.Add(1)
	go func() {
		time.Sleep(time.Second*1)
		sleepChan <- nil
	}()
	// ...

	produceMsg, err := json.Marshal(msg)
	if err != nil {
		// should cancel other async tasks
		return err
	}
	kafkaChan := fn.KafkaWriter.Write(ctx, &wg, []byte(msg.Name), []byte(produceMsg))
	
	wg.Wait()

	type result struct {
		C <-chan error
		Desc string
	}
	for _, e := range []result{
		{C: sleepChan, Desc: "sleep 1 second"},
		{C: kafkaChan, Desc: "produce message to kafka"},
	} {
		for err := range e.C {
			if err != nil {
				return fmt.Errorf("failed to %s, err[%+v]", e.Desc, err)
			}
		}
	}

	return nil
}

func main() {
	log.SetOutput(os.Stdout)

	f := sqslambda.NewFunction[ExampleContext](sqslambda.FunctionOptions[ExampleContext]{})
	f.Run(ExampleHandler)
}
