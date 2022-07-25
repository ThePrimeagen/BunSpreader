package queue

import (
	"sync/atomic"
	"time"

	"github.com/scryner/lfreequeue"
)

type InnerMessage struct {
    width int
    height int
    girth int
    depth int
    length int
    circumference int
}

type Message struct {
    message string
    another_property InnerMessage
}

type QueueMessage struct {
    Time int64
    Message Message
}

type Queue struct {
    Length int32
    queue lfreequeue.Queue
    backup lfreequeue.Queue
}

func NewQueue() *Queue {
    return &Queue {0, *lfreequeue.NewQueue(), *lfreequeue.NewQueue()}
}

func (q *Queue) Enqueue(node *QueueMessage) {
    atomic.AddInt32(&q.Length, 1)
    q.queue.Enqueue(node)
}

func (q *Queue) deque() (*QueueMessage, bool) {
    if atomic.LoadInt32(&q.Length) == 0 {
        return nil, false
    }

    atomic.AddInt32(&q.Length, -1)
    node, ok := q.queue.Dequeue()

    return node.(*QueueMessage), ok
}

func MakeTimestamp() int64 {
    return time.Now().UnixMilli()
}

func (q *Queue) EmptyQueue() {
    msg, count := emptyQueue(&q.backup)
    if count > 0 {
        atomic.AddInt32(&q.Length, -1 * count)
    }

    if msg != nil {
        q.backup.Enqueue(msg)
        return
    }

    msg, count = emptyQueue(&q.queue)
    if count > 0 {
        atomic.AddInt32(&q.Length, -1 * count)
    }

    if msg != nil {
        q.backup.Enqueue(msg)
    }
}

func emptyQueue(queue *lfreequeue.Queue) (*QueueMessage, int32) {
    var count int32
    count = 0
    now := MakeTimestamp()

    for {
        item, ok := queue.Dequeue()
        if !ok {
            break
        }

        node, _ := item.(*QueueMessage)

        // THIS IS SHITTY
        if node != nil && node.Time < now {
            count += 1
        } else {
            return node, count
        }
    }

    return nil, count
}



