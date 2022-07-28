package queue

import "testing"

func BenchmarkQueue(b *testing.B) {
	for i := 0; i < b.N; i++ {
		q := NewQueue()
		for i := 0; i < 100; i++ {
			for i := 0; i < 100; i++ {
				q.Enqueue(QueueMessage{Message: Message{message: "hello"}})
			}
			q.EmptyQueue()
		}
	}
}
