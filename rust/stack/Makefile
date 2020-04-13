target := static_size_stack
.PHONY: all clean ${target}
.DEFAULT_GOAL: ${target}
SRC := $(target).rs

all: ${target}

${target}:
	rustc ${SRC}

clean:
	@rm ${target}
