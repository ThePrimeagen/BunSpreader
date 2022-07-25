package main

import (
	"fmt"
	"net/http"
	"strconv"

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

        node := queue.NewNode(queue.QueueMessage {
            Time: queue.MakeTimestamp() + int64(tiq),
            Message: json,
        })

        q.Enqueue(node)
		c.String(200, fmt.Sprintf("time in queue will be %v", tiq))
    })

	r.GET("/status", func(c *gin.Context) {
        q.EmptyQueue()
		c.String(200, strconv.Itoa(q.Length))
	})

    r.Run("0.0.0.0:3000") // listen and serve on 0.0.0.0:3000
}
