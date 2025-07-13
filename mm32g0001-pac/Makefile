build:
	svd2rust --reexport-core-peripherals -i MM32G0001.svd
	rm -rf src
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt

