const std = @import("std");
const web = @import("zhp/src/zhp.zig");

const net = std.net;
const fs = std.fs;

fn Queue(comptime T: type) type {
    return struct {
        const Self = @This();
        const Node = struct {
            next: ?*Node,
            value: T,
        };

        head: ?*Node,
        tail: ?*Node,
        len: usize,
        allocator: std.mem.Allocator,

        pub fn init(allocator: std.mem.Allocator) !*Self {
            var queue = try allocator.create(Self);

            queue.allocator = allocator;
            queue.len = 0;
            queue.head = null;
            queue.tail = null;

            return queue;
        }

        pub fn enqueue(self: *Self, value: T) !void {
            var node = try self.allocator.create(Node);
            node.next = null;

            self.len += 1;
            node.value = value;

            if (self.head == null) {
                self.head = node;
                self.tail = node;
                return;
            }

            self.tail.?.next = node;
            self.tail = node;
        }

        pub fn deque(self: *Self) ?T {
            if (self.head == null) {
                return null;
            }

            var node = self.head;
            self.head = self.head.?.next;

            if (self.head == null) {
                self.tail = null;
            }

            const value = node.?.value;
            self.allocator.destroy(node.?);

            return value;
        }
    };
}

pub const io_mode = .evented;
pub const log_level = .info;

const MainHandler = struct {
    pub fn get(self: *MainHandler, request: *web.Request, response: *web.Response) !void {
        _ = self;
        _ = request;
        try response.headers.put("Content-Type", "text/plain");
        _ = try response.stream.write("Hello, World!");
    }

};

pub const routes = [_]web.Route{
    web.Route.create("home", "/", MainHandler),
};

pub const middleware = [_]web.Middleware{
    web.Middleware.create(web.middleware.LoggingMiddleware),
};

pub fn main() anyerror!void {


    // http://std.heap.page_allocator

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(!gpa.deinit());
    const allocator = gpa.allocator();

    var app = web.Application.init(allocator, .{.debug=true});
    defer app.deinit();

    try app.listen("0.0.0.0", 3000);
    try app.start();

    // var server = net.StreamServer.init(.{});
    // defer server.deinit();

    // try server.listen(net.Address.parseIp("127.0.0.1", 42070) catch unreachable);
    // std.log.warn("listening at {}\n", .{server.listen_address});

    // while (true) {
    //     const conn = try server.accept();
    //     defer conn.stream.close();

    //     var buff = std.mem.zeroes([1]u8);

    //     const read = try conn.stream.reader().read(&buff);
    //     if (read != 1) {
    //         unreachable;
    //     }

    //     buff[0] += 1;

    //     try conn.stream.writer().writeAll(buff[0..1]);
    // }
    //
}

