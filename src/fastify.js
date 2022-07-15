"use strict"

const fastify = require('fastify')({
    logger: false
});

class RingBuffer {
    constructor(size) {
        this.length = 0;
        this.data = new Array(size);
        this.head = 0;
        this.tail = 0;
        this.size = size;
    }

    enqueue(message) {
        let head_idx = this.head % this.size;
        if (head_idx === this.tail % this.size && this.length === this.size) {
            this.grow();
            head_idx = ++this.head;
        }

        this.length++;
        this.head++;
        this.data[head_idx] = message;
    }

    deque() {
        if (this.length === 0) {
            return;
        }

        this.length--;
        const out = this.data[this.tail % this.size];
        this.data[this.tail++] = null;

        return out;
    }

    peek() {
        if (this.length === 0) {
            return undefined;
        }

        return this.data[this.tail % this.size];
    }

    grow() {
        debugger
        const new_size = this.size + Math.min(this.size * 2, 20000);
        const new_data = new Array(new_size).fill(null);

        for (let i = 0, start = this.tail; start < this.head; ++i, ++start) {
            new_data[i] =  this.data[start % this.size];
        }

        this.tail = 0;
        this.head = this.length - 1;
        this.size = new_size;
        this.data = new_data;
    }
}

const queue = new RingBuffer();

function empty_queue() {
    const now = Date.now();
    while (queue.peek() !== undefined && queue.peek().time < now) {
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
    console.log("I AM LISTENING");
});

