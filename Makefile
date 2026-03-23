all:
	cargo run --release -- build build \
		--vendors data/android-vendors.json \
		--devices data/android.json \
		--vendors data/iphone-vendors.json \
		--devices data/iphone.json

fetch:
	cargo run --release -- fetch-lineage --devices data/android.json --vendors data/android-vendors.json
	cargo run --release -- fetch-iphone --devices data/iphone.json
	cargo run --release -- fetch-knox data/android.json --rules rules/knox.toml

serve:
	miniserve -i 127.0.0.1 --index index.html build
