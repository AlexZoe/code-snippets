target1 := static_size_stack
target1 := vec_stack
.PHONY: all clean ${target1} ${target2}
.DEFAULT_GOAL: ${target1}

all: ${target1} ${target2}

${target1}:
	@rustc $@.rs

${target2}:
	@rustc $@.rs

clean:
	@rm -rf ${target1}
	@rm -rf ${target2}
