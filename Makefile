generate:
	cargo build --release
	./target/release/main > vectors.json

generate-prg:
	cd XKCP
	git apply ../XKCP-patch.diff
