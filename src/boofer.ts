export type Message = {
    message: any,
    time: number,
}

type Node<T> = {
    value: T;
    next?: Node<T>;
}

export interface Queue<T> {
    readonly length: number;
    enqueue(time: T): void;
    peek(): T | undefined;
    deque(): T;
}

export class List<T> {
    private head?: Node<T>;
    private tail?: Node<T>;
    public length: number;
    constructor() {
        this.length = 0;
    }

    enqueue(time: T) {
        this.length++;
        const node = {value: time, next: undefined};
        if (!this.head) {
            this.head = this.tail = node;
            return;
        }

        this.tail.next = node;
        this.tail = node;
    }
    peek(): T | undefined {
        if (!this.head) {
            return undefined;
        }

        return this.head.value;
    }

    deque(): T {
        this.length--;
        if (!this.head) {
            return;
        }

        const node = this.head;
        this.head = this.head.next;
        node.next = undefined;

        return node.value;
    }
}

export class RingBuffer<T> {
    private data: T[];
    public length: number;
    private head: number;
    private tail: number;

    constructor(private size: number = 5) {
        this.length = 0;
        this.data = new Array(size);
        this.head = 0;
        this.tail = 0;
    }

    enqueue(message: T) {
        let head_idx = this.head % this.size;
        if (head_idx === this.tail % this.size && this.length === this.size) {
            this.grow();
            head_idx = ++this.head;
        }

        this.length++;
        this.head++;
        this.data[head_idx] = message;
    }

    deque(): T {
        if (this.length === 0) {
            return;
        }

        this.length--;
        const out = this.data[this.tail % this.size];
        this.data[this.tail++] = null;

        return out;
    }

    peek(): undefined | T {
        if (this.length === 0) {
            return undefined;
        }

        return this.data[this.tail % this.size];
    }

    private grow() {
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


