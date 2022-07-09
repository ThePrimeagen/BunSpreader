"use strict"

const fastify = require('fastify')({
    logger: false
});

const queue = [];

function empty_queue() {
    const now = Date.now();
    while (queue.length > 0 && queue[0].time < now) {
        queue.shift();
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
    queue.push(msg);

    return `time in queue will be ${time_in_queue}`;
});

fastify.get("/status", async (request, reply) => {
    empty_queue();
    return `${queue.length}`;
});

fastify.listen({ host: "0.0.0.0", port: 3000 }, (err, address) => {
});

