python update_path.py

cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/md_website.wasm --out-dir ./rust-out --target web