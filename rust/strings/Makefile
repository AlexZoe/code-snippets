target := string
.PHONY: clean ${target}
.DEFAULT_GOAL: ${target}
SRC := ${target}.rs

${target}:
	@echo '    ' Build $@
	@rustc ${SRC}

clean:
	@rm ${target}
