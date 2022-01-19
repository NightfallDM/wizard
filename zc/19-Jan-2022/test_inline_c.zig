const c = @cImport(@cInclude("xxx.h"));
const std = @import("std");

pub fn main() !void {
    const num = c.add_two(99);
    std.debug.print("num = {d}\n", .{num});
}
