all:
	RUST_BACKTRACE=1 cargo run --features 'watch serde_type'

run-release:
	RUST_BACKTRACE=1 cargo run --features 'serde_type' --release

release:
	cargo build --release --features 'serde_type'

deploy: release
	ssh log-iko-yo 'mkdir -p ~/outing-mining-rust'
	scp -C ../target/release/outing-mining-rust log-iko-yo:~/outing-mining-rust
	scp -C -r templates ../etc ../public log-iko-yo:~/outing-mining-rust

update:
	cargo update

clean:
	cargo clean
