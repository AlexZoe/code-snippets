target := reversed_single_linked
.PHONY: all clean ${target}
.DEFAULT_GOAL: ${target}
SRC := $(target).rs

all: ${target}

${target}:
	rustc ${SRC}

clean:
	@rm ${target}
