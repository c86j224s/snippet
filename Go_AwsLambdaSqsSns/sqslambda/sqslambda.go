// sqslambda package is a helper package writing a Lambda Function working with SQS and SNS.

package sqslambda

import (
	"context"
	"sync"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"

	"alss/kafkacli"
)

type Function[T any] struct {
	Wg sync.WaitGroup
	KafkaWriter *kafkacli.Writer
	Context T // user context object
}

type FunctionOptions[T any] struct {
	Context T

	UseKafkaWriter bool
	KafkaBrokers []string
	KafkaProduceTopic string
}

func NewFunction[T any](opt FunctionOptions[T]) *Function[T] {
	ret := &Function[T]{}
	if opt.UseKafkaWriter {
		ret.KafkaWriter = kafkacli.NewWriter(opt.KafkaBrokers, opt.KafkaProduceTopic)
	}
	
	return ret
}

func (f *Function[T]) Shutdown() {
	f.KafkaWriter.Shutdown()

	f.Wg.Wait()
}

func (f *Function[T]) Run(handler func(context.Context, *Function[T], string) error) {
	lambda.StartWithOptions(func(ctx context.Context, evt events.SQSEvent) (events.SQSEventResponse, error) {
		wg := sync.WaitGroup{}
		failureChannels := []<-chan *events.SQSBatchItemFailure{}
	
		for _, record := range evt.Records {
			// SNS message via SQS has MessageId and Body fields.
			messageId := record.MessageId
			body := record.Body

			failChan := make(chan *events.SQSBatchItemFailure, 1)
			failureChannels = append(failureChannels, failChan)

			// run goroutines for each records.
			wg.Add(1)
			go func() {
				defer wg.Done()

				// handler is expected to return a value or an error as its result.
				if err := handler(ctx, f, body); err != nil {
					failChan <- &events.SQSBatchItemFailure{ItemIdentifier: messageId}
				} else {
					failChan <- nil
				}
				close(failChan)
			}()
		}

		// waits all routines end.
		wg.Wait()

		// collects failures.
		failures := []events.SQSBatchItemFailure{}
		for _, failChan := range failureChannels {
			for fail := range failChan {
				if fail != nil {
					failures = append(failures, *fail)
				}
			}
		}

		return events.SQSEventResponse{BatchItemFailures: failures}, nil
	}, lambda.WithContext(context.Background()))

	
}