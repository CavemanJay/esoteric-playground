test:
	cargo watch -s "cargo nextest run && cargo t --doc"

docserver:
	browser-sync start -s target/doc --directory 

docs:
	cargo watch -s "cargo doc && browser-sync reload"
