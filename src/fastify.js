"use strict"

const fastify = require('fastify')({
    logger: false
});

class List {
    constructor() {
        this.length = 0;
        this.head = this.tail = undefined;
    }

    enqueue(time) {
        this.length++;
        const node = {time, next: undefined};
        if (!this.head) {
            this.head = this.tail = node;
            return;
        }

        this.tail.next = node;
        this.tail = node;
    }
    peek() {
        if (!this.head) {
            return undefined;
        }

        return this.head.time.time;
    }

    deque() {
        this.length--;
        if (!this.head) {
            return;
        }

        const node = this.head;
        this.head = this.head.next;
        node.next = undefined;
    }
}

const queue = new List();

function empty_queue() {
    const now = Date.now();
    const peeked = queue.peek();
    while (queue.peek() !== undefined && queue.peek() < now) {
        queue.deque();
    }
}

fastify.post("/json/:time_in_queue", async (request, reply) => {
    empty_queue();

    let time_in_queue = 15000;
    let json = request.body;

    const msg = {
        json,
        time: Date.now() + time_in_queue,
    };
    queue.enqueue(msg);

    return `time in queue will be ${time_in_queue}`;
});

fastify.get("/status", async (request, reply) => {
    empty_queue();
    return `${queue.length}`;
});

fastify.listen({ host: "0.0.0.0", port: 3000 }, (err, address) => {
});

