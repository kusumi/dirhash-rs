bin1:
	cargo build --release --features=squash1
	# cargo run --release --features=squash1 -- ...
bin2:
	cargo build --release --features=squash2
	# cargo run --release --features=squash2 -- ...
fmt:
	cargo fmt
	git status
clean:
	cargo clean
test1:
	cargo test --release --features=squash1
test2:
	cargo test --release --features=squash2
lint1:
	cargo clippy --release --fix --features=squash1
	git status
lint2:
	cargo clippy --release --fix --features=squash2
	git status

xxx1:	fmt lint1 test1
xxx2:	fmt lint2 test2
