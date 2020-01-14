all:
	@rm -f c-test
	gcc -c c-test.c
	ld -Ttext=40000000 --entry=main c-test.o -o c-test
	cp c-test ../../../build/bins/c-test
	@rm -f c-test.o
