rm -rf /home/harman/genun_certification/wasm_files
mkdir /home/harman/genun_certification/wasm_files

cargo build --target wasm32-unknown-unknown --release --package icrc7
candid-extractor target/wasm32-unknown-unknown/release/icrc7.wasm > /home/harman/genun_certification/src/icrc7/icrc7.did || true
mv /home/harman/genun_certification/target/wasm32-unknown-unknown/release/icrc7.wasm /home/harman/genun_certification/wasm_files
ic-wasm /home/harman/genun_certification/wasm_files/icrc7.wasm -o /home/harman/genun_certification/wasm_files/icrc7.wasm metadata candid:service -f /home/harman/genun_certification/src/icrc7/icrc7.did -v public
gzip /home/harman/genun_certification/wasm_files/icrc7.wasm



cargo build --target wasm32-unknown-unknown --release --package genun_backend
candid-extractor target/wasm32-unknown-unknown/release/genun_backend.wasm > /home/harman/genun_certification/src/genun_backend/genun_backend.did || true