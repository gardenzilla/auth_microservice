 
.PHONY: release, test, dev, run, bench

release:
	cargo update
	cargo build --release
	strip target/release/auth_microservice

build:
	cargo build

run:
	HASH_SECRET=DEV_SECRET cargo run

dev:
	# . ./ENV.sh; backper
	cargo run;

test:
	cargo test

bench:
	cargo bench