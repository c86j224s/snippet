// kakfacli package offers a client for kafka. producer only yet.

package kafkacli

import (
	"context"
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/segmentio/kafka-go"
)

// Writer struct is a thin wrapper of kafka.Writer.
// This supports transaction maangement for produced messages.
//	Key in kafka message is required if want to manage transaction.
type Writer struct {
	kafka.Writer
	TxMap map[string]chan<- error
	TxMtx sync.Mutex
}

func NewWriter(brokers []string, produceTopic string) *Writer {
	w := &Writer{
		Writer: kafka.Writer{
			Addr:     kafka.TCP(brokers...),
			Topic:    produceTopic,
			Balancer: &kafka.LeastBytes{},
			Async:    true,
		},
		TxMap: make(map[string]chan<- error),
		TxMtx: sync.Mutex{},
	}

	w.Completion = func(messages []kafka.Message, err error) {
		// check success of message production asynchronously.

		for _, msg := range messages {
			// check messages only that those keys are offered.
			if len(msg.Key) != 0 {
				if tx := w.claimTx(msg.Key); tx != nil {
					*tx <- err
					close(*tx)
				}
			}

			log.Println("writer completion, msg[", msg, "], err[", err, "]")
		}
	}

	return w
}

func (w *Writer) Shutdown() {
	if err := w.Writer.Close(); err != nil {
		log.Println("writer close error, err[", err, "]")
	}
}

func (w *Writer) Write(ctx context.Context, wg *sync.WaitGroup, key []byte, val []byte) <-chan error {
	ret := make(chan error, 1)

	var waiter <-chan error

	// if key is not offered, no check success of message production asynchronously.
	if len(key) != 0 {
		waiter = w.storeTx(key)
	} else {
		wsender := make(chan error, 1)
		waiter = wsender
		close(wsender)
	}

	msg := kafka.Message{Key: key, Value: val}
	if err := w.WriteMessages(ctx, msg); err != nil {
		w.destroyTx(key)

		ret <- err
		return ret
	}

	wg.Add(1)
	go func() {
		defer func() {
			close(ret)
			wg.Done()
		}()

		select {
		case err := <-waiter:
			ret <- err

		case <-time.After(10 * time.Second):
			ret <- fmt.Errorf("channel timeout, msg[%+v]", msg)
		}
	}()

	return ret
}

func (w *Writer) storeTx(key []byte) <-chan error {
	k := string(key)

	waiter := make(chan error, 1)

	w.TxMtx.Lock()
	defer w.TxMtx.Unlock()

	if prevWaiter, exists := w.TxMap[k]; exists {
		close(prevWaiter)
	}

	w.TxMap[k] = waiter

	return waiter
}

func (w *Writer) claimTx(key []byte) *chan<- error {
	k := string(key)

	w.TxMtx.Lock()
	defer w.TxMtx.Unlock()

	if v, exists := w.TxMap[k]; !exists {
		return nil
	} else {
		delete(w.TxMap, k)
		return &v
	}
}

func (w *Writer) destroyTx(key []byte) {
	k := string(key)

	w.TxMtx.Lock()
	defer w.TxMtx.Unlock()

	if prevWaiter, exists := w.TxMap[k]; exists {
		close(prevWaiter)
		delete(w.TxMap, k)
	}
}
