"use strict"

const fastify = require('fastify')({
    logger: false
});

const cluster = require('cluster');
const numCPUs = require('os').cpus().length;

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

const port = process.env.PORT || 3000

if (cluster.isPrimary) {
  console.log(`Primary ${process.pid} is running`);
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
    cluster.on('exit', (worker, code, signal) => {
      if (signal) {
        console.log(`worker was killed by signal: ${signal}`);
      } else if (code !== 0) {
        console.log(`worker exited with error code: ${code} spawning new worker`);
        cluster.fork();
      } else {
        console.log('worker success!');
      }
    });
  }
} else {
  fastify.listen({port: port, host: "::"}, (err, address) => {
    if (err) {
      console.error(err)
      process.exit(1)
    }
    console.log(`Server listening at ${address}`)
  });
}
