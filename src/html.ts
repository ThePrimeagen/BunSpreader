const queue = new RingBuffer();

function empty_queue() {
    const now = Date.now();
    console.log("peek", queue.peek(), now);
    while (queue.peek() !== undefined && queue.peek() < now) {
        queue.deque();
    }
}

export default {
    port: 3000,
    async fetch(request: Request) {
        empty_queue();

        if (request.url.includes("status")) {
            return new Response(`${queue.length}`);
        }

        const params = request.url.split("json/")[1];
        let time_in_queue = 5000;
        if (params) {
            time_in_queue = params.split("/").map(x => +x)[0];
        }

        try {
            const json = await request.json();
            const msg = {
                message: json,
                time: Date.now() + time_in_queue,
            };
            queue.enqueue(msg);
        } catch (e) {
            console.error("unable to parse json", e);
        }

        return new Response(`time in queue will be ${time_in_queue}`);
    },
};
