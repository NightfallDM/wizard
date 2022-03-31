obj-m := simple_print.o

modules:
	$(MAKE) -C /lib/modules/`uname -r`/build M=`pwd` modules
clean:
	# rm -rf !(Makefile||*.c)
	rm -rf .m*
	rm -rf .s*
	rm -rf .M*
