all:
	@trash c-test
	gcc -c c-test.c
	ld -Ttext=40000000 --entry=main c-test.o -o c-test
	cp c-test ../../build/c-test
	@trash c-test.o
