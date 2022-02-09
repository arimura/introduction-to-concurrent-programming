kick-rust/%:
	rustc $*/main.rs && ./main

kick-c/%:
	clang $*/main.c && ./a.out