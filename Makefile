all:
	cargo run --release -- build build \
		--vendors data/android-vendors.json \
		--devices data/android.json \
		--vendors data/iphone-vendors.json \
		--devices data/iphone.json

fetch:
	cargo run --release -- fetch-lineage -H cache/lineage-index.html -T cache/lineage-wiki-main.tar.gz --devices data/android.json --vendors data/android-vendors.json
	cargo run --release -- fetch-iphone -i cache/eol-iphone.json --devices data/iphone.json
	cargo run --release -- fetch-knox -i cache/knox.json data/android.json --rules rules/knox.toml

serve:
	miniserve -i 127.0.0.1 --index index.html build
