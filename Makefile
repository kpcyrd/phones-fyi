all:
	cargo run --release -- build build \
		--vendors data/android-vendors.json \
		--devices data/android.json \
		--vendors data/iphone-vendors.json \
		--devices data/iphone.json

fetch:
	cargo run --release -- fetch-lineage -H lineage-index.html -T lineage-wiki-main.tar.gz --devices data/android.json --vendors data/android-vendors.json
	cargo run --release -- fetch-iphone -i eol-iphone.json --devices data/iphone.json
