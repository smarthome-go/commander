build:
	RUSTUP_TOOLCHAIN=stable CARGO_TARGET_DIR=target cargo build --release

install: build
	sudo cp ./target/release/commander /usr/bin
	sudo cp ./commander@.service /etc/systemd/system
	sudo systemctl daemon-reload

clean:
	rm -rf target
