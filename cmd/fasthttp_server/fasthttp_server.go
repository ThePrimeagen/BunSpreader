package main

import (
	"fmt"
	"net/http"
	"strconv"
	"encoding/json"

	"github.com/ThePrimeagen/BunSpreader/pkg/queue"
	"github.com/valyala/fasthttp"
	"github.com/valyala/fasthttprouter"
)

func main () {
	r := fasthttprouter.New()
	q := queue.NewQueue()

	r.POST("/json/:timeInQueue", func(ctx *fasthttp.RequestCtx, ps fasthttprouter.Params) {
        q.EmptyQueue()

		timeInQueue := ps.ByName("timeInQueue")
        var jsonMsg queue.Message

		body := ctx.PostBody()
		err := json.Unmarshal(body, &jsonMsg)
		if err != nil {
			ctx.Error(err.Error(), http.StatusBadRequest)
			return
		}

        tiq, err := strconv.Atoi(timeInQueue)
        if err != nil {
			ctx.Error(err.Error(), http.StatusBadRequest)
			return
        }

        message := queue.QueueMessage {
            Time: queue.MakeTimestamp() + int64(tiq),
            Message: jsonMsg,
        }

        q.Enqueue(&message)
		fmt.Fprintf(ctx, "time in queue will be %v", tiq)
    })

	r.GET("/status", func(ctx *fasthttp.RequestCtx, ps fasthttprouter.Params) {
        q.EmptyQueue()
		ctx.Write([]byte(strconv.Itoa(q.Length)))
	})

	fasthttp.ListenAndServe("0.0.0.0:3000", r.Handler);
}
