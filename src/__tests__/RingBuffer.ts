import { RingBuffer } from "../html";

test("Ring Buffer", function() {
    const buffer = new RingBuffer<number>();

    buffer.enqueue(5);
    buffer.enqueue(6);
    buffer.enqueue(7);
    buffer.enqueue(69);
    buffer.enqueue(420);
    buffer.enqueue(42069);

    expect(buffer.length).toEqual(6);
    expect(buffer.peek()).toEqual(5);
    expect(buffer.deque()).toEqual(5);
    expect(buffer.length).toEqual(5);

    expect(buffer.deque()).toEqual(6);
    expect(buffer.deque()).toEqual(7);
    expect(buffer.deque()).toEqual(69);
    expect(buffer.deque()).toEqual(420);
    expect(buffer.deque()).toEqual(42069);
    expect(buffer.length).toEqual(0);
    expect(buffer.deque()).toEqual(undefined);
    expect(buffer.peek()).toEqual(undefined);
});

