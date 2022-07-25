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

        return this.head.time;
    }

    deque() {
        this.length--;
        if (!this.head) {
            return;
        }

        const node = this.head;
        this.head = this.head.next;
        node.next = undefined;
        return node;
    }
}

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
        console.log("grow#start", this.head, this.tail, this.length, this.size);
        const new_size = this.size + Math.min(this.size * 2, 20000);
        const new_data = new Array(new_size).fill(null);

        for (let i = 0, start = this.tail; start < this.head; ++i, ++start) {
            new_data[i] =  this.data[start % this.size];
        }

        this.tail = 0;
        this.head = this.length - 1;
        this.size = new_size;
        this.data = new_data;
        console.log("grow#end", this.head, this.tail, this.length, this.size);
    }
}

let queue = new List();
if (process.argv[2] === "buf" || process.env["QUEUE"] === "buf") {
    console.log("using buf");
    queue = new RingBuffer(5);
} else {
    console.log("using list");
}

function empty_queue() {
    const now = Date.now();
    while (queue.peek() !== undefined && queue.peek().time < now) {
        queue.deque();
    }
}

let count = 0;
fastify.post("/json/:time_in_queue", function json_handler(request, reply) {
    count++;

    if (count % 100000 === 0) {
        console.log("count", count);
    }

    try {
        empty_queue();

        let time_in_queue = 15000;
        let json = request.body;

        const msg = {
            json,
            time: Date.now() + time_in_queue,
        };
        queue.enqueue(msg);
    } catch(e) {
        console.log("ERROR", e);
    }

    return `time in queue will be ${time_in_queue}`;
});

fastify.get("/status", function status_handler(request, reply) {
    empty_queue();
    return `${queue.length}`;
});

fastify.listen({ host: "0.0.0.0", port: 3000 }, (err, address) => {
    console.log("I AM LISTENING");
});

