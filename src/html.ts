import { Message, RingBuffer, List, Queue } from "./boofer";

let queue: Queue<Message> = new List();
if (process.argv[2] === "buf" || process.env["QUEUE"] === "buf") {
    queue = new RingBuffer(5);
}

function empty_queue() {
    const now = Date.now();
    while (queue.peek() !== undefined && queue.peek().time < now) {
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
        }

        return new Response(`time in queue will be ${time_in_queue}`);
    },
};
