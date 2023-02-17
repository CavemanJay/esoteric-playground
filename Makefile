docserver:
	browser-sync start -s target/doc --directory 

docs:
	cargo watch -s "cargo doc && browser-sync reload"
