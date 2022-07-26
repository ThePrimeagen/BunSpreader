package chan_queue

import (
	"context"
	"sync"
	"time"
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

type Node struct {
    data QueueMessage
    next *Node
}

type Queue struct {
    length int
    head *Node
    tail *Node
    lock sync.Mutex
}

func NewQueue() *Queue {
    return &Queue {0, nil, nil, sync.Mutex{}}
}

func (q *Queue) Run(ctx context.Context, wg *sync.WaitGroup, in chan *QueueMessage, out chan int, empty chan bool, status chan bool) {
    defer wg.Done()

    for {
        select {
        case message := <- in:
            q.enqueue(message)
        case <- empty:
            q.emptyQueue()
        case <- status:
            q.emptyQueue()
            out <- q.length
        case <- ctx.Done():
			return
        }
    }
}

func (q *Queue) enqueue(message *QueueMessage) {
    node := NewNode(*message)
    q.length += 1

    if q.head == nil {
        q.head = node
        q.tail = node
        return
    }

    q.tail.next = node
    q.tail = q.tail.next
}

func (q *Queue) deque() *Node {

    if q.head == nil {
        return nil
    }

    q.length -= 1

    out := q.head
    q.head = q.head.next

    out.next = nil
    return out
}

func MakeTimestamp() int64 {
    return time.Now().UnixMilli()
}

func (q *Queue) emptyQueue() {
    now := MakeTimestamp()

    for {
        node := q.head
        if node != nil && node.data.Time < now {
            q.deque()
        } else {
            break;
        }
    }
}

func NewNode(data QueueMessage) *Node {
    return &Node {
        data: data,
        next: nil,
    }
}
