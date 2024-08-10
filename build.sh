rm -rf wasm_files
mkdir wasm_files

cargo build --target wasm32-unknown-unknown --release --package icrc7
candid-extractor target/wasm32-unknown-unknown/release/icrc7.wasm > src/icrc7/icrc7.did || true
mv target/wasm32-unknown-unknown/release/icrc7.wasm wasm_files
ic-wasm wasm_files/icrc7.wasm -o wasm_files/icrc7.wasm metadata candid:service -f src/icrc7/icrc7.did -v public
gzip wasm_files/icrc7.wasm



cargo build --target wasm32-unknown-unknown --release --package genun_backend
candid-extractor target/wasm32-unknown-unknown/release/genun_backend.wasm > src/genun_backend/genun_backend.did || true