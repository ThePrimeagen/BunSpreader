const std = @import("std");
const http = @import("apple_pie.zig");
const router = http.router;

// use evented mode for event loop support
pub const io_mode = .evented;

// optional root constant to define max stack buffer size per request
pub const buffer_size: usize = 4096;
// optional root constant to define max header size per request
pub const request_buffer_size: usize = 4096;

var m = std.Thread.Mutex{};

const InnerJsonMessage = struct {
    width: i32,
    height: i32,
    girth: i32,
    depth: i32,
    length: i32,
    circumference: i32,
};

const JsonMessage = struct {
    message: []u8,
    another_property: InnerJsonMessage,
};

const Message = struct {
    time: u64,
    message: JsonMessage,
};

const MessageQueue = std.TailQueue(Message);

/// Context variable, accessible by all handlers, allowing to access data objects
/// without requiring them to be global. Thread-safety must be handled by the user.
const Context = struct {
    queue: *MessageQueue,
    allocator: std.mem.Allocator,
};

pub fn main() !void {
    var gpa = std.heap.c_allocator;

    var q = try gpa.create(MessageQueue);
    q.first = null;
    q.last = null;

    var my_context: Context = .{
        .queue = q,
        .allocator = gpa,
    };

    const builder = router.Builder(*Context);

    try http.listenAndServe(
        gpa,
        try std.net.Address.parseIp("0.0.0.0", 3000),
        &my_context,
        comptime router.Router(*Context, &.{
            builder.get("/status", null, status),
            builder.post("/json/:name", []const u8, jsonMessage),
        }),
    );
}

fn emptyQueue(allocator: std.mem.Allocator, q: *MessageQueue) void {
    var now = std.time.milliTimestamp();
    while (q.first) |first| {
        // std.log.warn("time is {} peek is {}", .{now, first.data.time});
        if (first.data.time <= now) {
            if (q.popFirst()) |item| {
                allocator.destroy(item);
            }
        } else {
            break;
        }
    }
}

fn status(c: *Context, response: *http.Response, request: http.Request , _: ?*const anyopaque) !void {
    m.lock();
    emptyQueue(c.allocator, c.queue);
    const len = c.queue.len;
    m.unlock();
    _ = request;
    try response.writer().print("{}", .{len});
}

fn jsonMessage(c: *Context, response: *http.Response, request: http.Request, captures: ?*const anyopaque) !void {

    _ = request;
    const name = @ptrCast(
        *const []const u8,
        @alignCast(
            @alignOf(*const []const u8),
            captures,
        ),
    );

    const body = request.body();
    var tokens = std.json.TokenStream.init(body);
    const s = try std.json.parse(JsonMessage, &tokens, .{
        .allocator = c.allocator,
    });

    // std.debug.print("{}\n", .{s});

    const time_in_queue = try std.fmt.parseInt(u32, name.*, 10);

    var item = try c.allocator.create(Message);
    item.time = @intCast(u64, std.time.milliTimestamp()) + time_in_queue;
    item.message = s;

    var node = try c.allocator.create(MessageQueue.Node);
    node.data = item.*;
    node.next = null;
    node.prev = null;

    m.lock();
    emptyQueue(c.allocator, c.queue);
    c.queue.append(node);
    m.unlock();

    try response.writer().print("time in queue will be {}", .{time_in_queue});
}

