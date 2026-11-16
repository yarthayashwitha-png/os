CC ?= gcc
CFLAGS ?= -O3 -Wall -Wextra -std=c99
LDFLAGS ?= -lm
TARGET = research-lab

all: $(TARGET)

$(TARGET): research_lab.c
	$(CC) $(CFLAGS) -o $@ $< $(LDFLAGS)

install: $(TARGET)
	install -d /usr/local/bin
	install -m 755 $(TARGET) /usr/local/bin/$(TARGET)

clean:
	rm -f $(TARGET) research_results.csv

.PHONY: all install clean
