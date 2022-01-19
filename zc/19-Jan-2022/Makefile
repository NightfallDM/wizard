# At this time i do not family with zig build system,So instead use make to do the auto-build work
# TODO: switch to the zig build system

test_inline_c: libxxx.a test_inline_c.zig
	zig build-exe test_inline_c.zig -lc -L./ -lxxx -I./

libxxx.a: xxx.c xxx.h
	gcc -c -O3 xxx.c
	ar -crs libxxx.a xxx.o

dump: xxx.o
	objdump -D xxx.o > xxx_objdump

clean:
	rm -rf zig-cache *.o *_objdump *.a
