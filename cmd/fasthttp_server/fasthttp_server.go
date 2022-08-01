package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"sync"

	queue "github.com/ThePrimeagen/BunSpreader/pkg/pool_queue"
	"github.com/valyala/fasthttp"
	"github.com/valyala/fasthttprouter"
)


func getPools() (sync.Pool, sync.Pool) {
	messagePool := sync.Pool{
		New: func() interface{} {
			return &queue.Message{}
	}}
	queueMessagePool := sync.Pool{
		New: func() interface{} {
			return &queue.QueueMessage{}
	}}

	// fill some pools
	ms := make([]*queue.Message, 1000000)
	qms := make([]*queue.QueueMessage, 1000000)
	for i := 0; i < 1000000; i++ {
		ms[i] = messagePool.Get().(*queue.Message)
		qms[i] = queueMessagePool.Get().(*queue.QueueMessage)
	}
	for i := 0; i < 1000000; i++ {
		messagePool.Put(ms[i])
		queueMessagePool.Put(qms[i])
	}
	return messagePool, queueMessagePool
}

func main () {
	r := fasthttprouter.New()
	messagePool, queueMessagePool := getPools()
	q := queue.NewQueue(messagePool, queueMessagePool)

	r.POST("/json/:timeInQueue", func(ctx *fasthttp.RequestCtx, ps fasthttprouter.Params) {
        q.EmptyQueue()

		timeInQueue := ps.ByName("timeInQueue")
        jsonMsg := messagePool.Get().(*queue.Message)

		body := ctx.PostBody()
		err := json.Unmarshal(body, jsonMsg)
		if err != nil {
			ctx.Error(err.Error(), http.StatusBadRequest)
			return
		}

        tiq, err := strconv.Atoi(timeInQueue)
        if err != nil {
			ctx.Error(err.Error(), http.StatusBadRequest)
			return
        }

		message := queueMessagePool.Get().(*queue.QueueMessage)
        message.Time = queue.MakeTimestamp() + int64(tiq)
		message.Message = jsonMsg

        q.Enqueue(message)
		fmt.Fprintf(ctx, "time in queue will be %v", tiq)
    })

	r.GET("/status", func(ctx *fasthttp.RequestCtx, ps fasthttprouter.Params) {
        q.EmptyQueue()
		ctx.Write([]byte(strconv.Itoa(q.Length)))
	})

	fasthttp.ListenAndServe("0.0.0.0:3000", r.Handler);
}
