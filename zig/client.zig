const std = @import("std");
const net = std.net;
const fs = std.fs;

pub fn main() anyerror!void {
    var count: u8 = 0;
    while (true) {

        var conn = try std.net.tcpConnectToAddress(net.Address.parseIp("127.0.0.1", 42070) catch unreachable);
        defer conn.close(); // does this work on loop?

        try conn.writer().writeByte(count);

        var buff = std.mem.zeroes([1]u8);
        const read = try conn.reader().read(&buff);
        if (read != 1) {
            std.log.err("Unable to read anything from the buffer.  Life must end\n", .{});
            unreachable;
        }

        count = buff[0];

        std.log.warn("I have a new value for count {}\n", .{count});
    }
}

test "basic test" {
    try std.testing.expectEqual(10, 3 + 7);
}

