curl https://sh.rustup.rs -sSf | bash -s -- -y --no-modify-path
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

wasm-pack build
export NODE_OPTIONS=--openssl-legacy-provider
