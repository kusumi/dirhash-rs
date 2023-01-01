bin1:
	cargo build --features=squash1
	# cargo run --features=squash1 -- ...
bin2:
	cargo build --features=squash2
	# cargo run --features=squash2 -- ...
fmt:
	cargo fmt
	git status
clean:
	cargo clean
lint1:
	cargo clippy --fix --features=squash1
	git status
lint2:
	cargo clippy --fix --features=squash2
	git status
