package main

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"sync"
	"syscall"

	"github.com/ThePrimeagen/BunSpreader/pkg/chan_queue"
	"github.com/gin-gonic/gin"
)

func main() {
    gin.SetMode(gin.ReleaseMode)

	c := make(chan os.Signal, 2)
	ctx, cancel := context.WithCancel(context.Background())
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		defer cancel()
		<-c
	}()

	enqueue := make(chan *chan_queue.QueueMessage)
	qLen := make(chan int)
	qEmpty := make(chan bool)
	qStatus := make(chan bool)
    q := chan_queue.NewQueue()

	var wg sync.WaitGroup
	wg.Add(1)
	go q.Run(ctx, &wg, enqueue, qLen, qEmpty, qStatus)

	r := gin.New()

    r.POST("/json/:timeInQueue", func(c *gin.Context) {
		qEmpty <- true

		timeInQueue := c.Param("timeInQueue")
        var json chan_queue.Message
		if err := c.ShouldBindJSON(&json); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

        tiq, err := strconv.Atoi(timeInQueue)
        if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
        }

        message := chan_queue.QueueMessage {
            Time: chan_queue.MakeTimestamp() + int64(tiq),
            Message: json,
        }

		enqueue <- &message
		c.String(200, fmt.Sprintf("time in queue will be %v", tiq))
    })

	r.GET("/status", func(c *gin.Context) {
		qStatus <- true
        length := <- qLen
		c.String(200, strconv.Itoa(length))
	})

    go r.Run("0.0.0.0:3000") // listen and serve on 0.0.0.0:3000

	wg.Wait()
	os.Exit(0)
}
