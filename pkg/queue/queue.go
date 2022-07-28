package queue

import (
	"sync"
	"time"
)

type InnerMessage struct {
	width         int
	height        int
	girth         int
	depth         int
	length        int
	circumference int
}

type Message struct {
	message          string
	another_property InnerMessage
}

type QueueMessage struct {
	Time    int64
	Message Message
}

type Node struct {
	data QueueMessage
	next *Node
}

type Queue struct {
	Length int32

	head *Node
	tail *Node
	lock sync.Mutex
	pool sync.Pool
}

func NewQueue() *Queue {
	return &Queue{0, nil, nil, sync.Mutex{}, sync.Pool{}}
}

func (q *Queue) Enqueue(qm QueueMessage) {
	var node *Node
	node, ok := q.pool.Get().(*Node)
	if !ok {
		node = &Node{}
	}
	node.data = qm

	q.lock.Lock()
	defer q.lock.Unlock()

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
			q.pool.Put(q.deque())
		} else {
			break
		}
	}
}
