package main

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

type InnerMessage struct {
}

type Message struct {

}

func main() {
	r := gin.Default()

    r.POST("/json/:timeInQueue", func(c *gin.Context) {
		timeInQueue := c.Param("timeInQueue")
        var json Message
		if err := c.ShouldBindJSON(&json); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
    })

	r.GET("/status", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	r.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}
