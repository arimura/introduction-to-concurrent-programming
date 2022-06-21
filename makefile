kick-rust/%:
	rustc $*/main.rs && ./main

kick-c/%:
	clang $*/*.c && ./a.out

docker-build:
	docker build -t itcp-rust .

docker-run:
	docker run -it -v $(shell pwd):/usr/src/introduction-to-concurrent-programming itcp-rust
