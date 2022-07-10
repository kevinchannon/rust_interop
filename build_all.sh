cargo build

cbindgen --config cbindgen.toml --crate rust_interop --output src/rust_interop.h --lang c

cmake --build build

