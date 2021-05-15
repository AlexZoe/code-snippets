defaul_goal: main

main:
	cargo build --lib --release
	g++ -o main main.cpp target/release/libcall_from_c.a -lpthread -ldl

analyze: main
	valgrind --leak-check=full --show-leak-kinds=all ./main

clean:
	@cargo clean
	@rm -rf main

