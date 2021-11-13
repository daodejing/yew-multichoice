FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN cargo install trunk wasm-bindgen-cli