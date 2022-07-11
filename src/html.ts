type Message = {
    message: any,
    time: number,
}
type Node = {
    time: Message;
    next?: Node;
}

class List {
    private head?: Node;
    private tail?: Node;
    public length: number;
    constructor() {
        this.length = 0;
    }

    enqueue(time: Message) {
        this.length++;
        const node = {time, next: undefined};
        if (!this.head) {
            this.head = this.tail = node;
            return;
        }

        this.tail.next = node;
        this.tail = node;
    }
    peek(): number | undefined {
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
