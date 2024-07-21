const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{
        .name = "test",
        .link_libc = true,
        .target = b.host,
    });

    exe.addCSourceFile(.{ .file = b.path("main.c") });

    b.installArtifact(exe);
}
