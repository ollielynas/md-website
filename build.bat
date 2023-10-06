rem run:
rem ./update -r 
rem to build in release mode

echo off
set arg1=%1
set arg2=%2
shift
shift



python update_path.py

cargo +nightly build --target wasm32-unknown-unknown %arg1%
if %arg1% == -r (wasm-bindgen target/wasm32-unknown-unknown/release/md_website.wasm --out-dir ./rust-out --target web) else (wasm-bindgen target/wasm32-unknown-unknown/debug/md_website.wasm --out-dir ./rust-out --target web)
