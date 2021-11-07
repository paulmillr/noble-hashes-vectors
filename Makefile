generate:
	cargo build --release
	./target/release/main > vectors.json

generate-prg:
	echo 'If you are on MacOS with ARM arch, comment-out -march=native'
	cd XKCP && git apply ../xkcp-prg.diff && make generic64/UnitTests
	./XKCP/bin/generic64/UnitTests --KeccakPRG > keccak-prg-output.txt
	node prg-gen.js > keccak-prg.json
