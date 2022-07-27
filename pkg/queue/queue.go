package queue

import (
	"sync"
	"time"
)

type InnerMessage struct {
    Width int   `json:"width"`
    Height int  `json:"height"`
    Girth int   `json:"girth"`
    Depth int   `json:"depth"`
    Length int  `json:"length"`
    Circumference int   `json:"circumference"`
}

type Message struct {
    Message string  `json:"message"`
    AnotherProperty InnerMessage `json:"another_property"`
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
    Length int

    head *Node
    tail *Node
    lock sync.Mutex
}

func NewQueue() *Queue {
    return &Queue {0, nil, nil, sync.Mutex{}}
}

func (q *Queue) Enqueue(message *QueueMessage) {
    q.lock.Lock()
    defer q.lock.Unlock()

	node := NewNode(*message)
    q.Length += 1

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

    q.Length -= 1

    out := q.head
    q.head = q.head.next

    out.next = nil
    return out
}

func MakeTimestamp() int64 {
    return time.Now().UnixMilli()
}

func (q *Queue) length() {
}

func (q *Queue) EmptyQueue() {
    q.lock.Lock()
    defer q.lock.Unlock()

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
