run survie:
    cargo run --package survie --features hotreload

build survie:
    cargo build --release --package survie

serve-web survie: (build-web survie)
    miniserve --index index.html public

build-web survie:
    cargo build --release --package survie --target wasm32-unknown-unknown
    wasm-bindgen --target web --no-typescript --out-dir public/wasm --out-name survie \
        target/wasm32-unknown-unknown/release/survie.wasm
