bin1:
	cargo build --release --features=squash1
	# cargo run --release --features=squash1 -- ...
bin2:
	cargo build --release --features=squash2
	# cargo run --release --features=squash2 -- ...
clean:
	cargo clean
fmt:
	cargo fmt
	git status
lint1:
	cargo clippy --release --fix --all --features=squash1
	git status
plint1:
	cargo clippy --release --fix --all --features=squash1 -- -W clippy::pedantic
	git status
lint2:
	cargo clippy --release --fix --all --features=squash2
	git status
plint2:
	cargo clippy --release --fix --all --features=squash2 -- -W clippy::pedantic
	git status
test1:
	cargo test --release --features=squash1
test2:
	cargo test --release --features=squash2

xxx1:	fmt lint1 test1
xxx2:	fmt lint2 test2
