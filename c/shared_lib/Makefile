all:
	gcc -c -Wall -Werror -fpic myshared.c
	gcc -shared -o libmyshared.so myshared.o
	gcc -L$(shell pwd) -Wl,-rpath=$(shell pwd) -Wall -o test main.c -lmyshared

clean:
	rm -rf myshared.o
	rm -rf libmyshared.so
	rm -rf test

