package main

import (
	"fmt"
	"net/http"
	"strconv"
	"sync/atomic"

	"github.com/ThePrimeagen/BunSpreader/pkg/queue"
	"github.com/gin-gonic/gin"
)

func main() {
    gin.SetMode(gin.ReleaseMode)

	r := gin.New()
    q := queue.NewQueue()


    r.POST("/json/:timeInQueue", func(c *gin.Context) {
        q.EmptyQueue()

		timeInQueue := c.Param("timeInQueue")
        var json queue.Message
		if err := c.ShouldBindJSON(&json); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

        tiq, err := strconv.Atoi(timeInQueue)
        if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
        }

        message := queue.QueueMessage {
            Time: queue.MakeTimestamp() + int64(tiq),
            Message: json,
        }

        q.Enqueue(&message)
		c.String(200, fmt.Sprintf("time in queue will be %v", tiq))
    })

	r.GET("/status", func(c *gin.Context) {
        q.EmptyQueue()
		len32 := int32(q.Length)
		c.String(200, strconv.Itoa(int(atomic.LoadInt32(&len32))))
	})

    r.Run("0.0.0.0:3000") // listen and serve on 0.0.0.0:3000
}
