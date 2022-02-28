kick-rust/%:
	rustc $*/main.rs && ./main

kick-c/%:
	clang $*/*.c && ./a.out