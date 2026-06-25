

dev:	
	cargo watch -q -c -x run
test:	
	cargo watch -x test
build:	
	cargo build --release