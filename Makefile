
all: histogram

histogram: main.c
	$(CC) --std=c11 -o histogram main.c


