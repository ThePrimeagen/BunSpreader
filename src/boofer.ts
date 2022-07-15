export type Message = {
    message: any,
    time: number,
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


