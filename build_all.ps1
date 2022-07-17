cargo build

cbindgen -q --config cbindgen.toml --crate rust_interop --output src/rust_interop.h --lang c

clang .\src\main.c -o interop.exe -L .\target\debug -lrust_interop -lws2_32 -lAdvapi32 -lUserEnv