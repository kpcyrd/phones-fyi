all:
	cargo run --release -- build build \
		--vendors data/android-vendors.json \
		--devices data/android.json \
		--vendors data/iphone-vendors.json \
		--devices data/iphone.json

fetch:
	cargo run --release -- fetch-linage -i linage-index.html --devices data/android.json --vendors data/android-vendors.json
	cargo run --release -- fetch-iphone -i eol-iphone.json --devices data/iphone.json
