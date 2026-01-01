all:
	cargo run --release -- build build \
		--vendors data/android-vendors.json \
		--devices data/android.json \
		--vendors data/iphone-vendors.json \
		--devices data/iphone.json
