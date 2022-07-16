=Summary=

This is a simple project to play with some C-Rust interop things.

One principle that it's trying to adhere to is that any resources allocated by Rust are managed by Rust and not released to the C-side of things.  To do this, an opaque-handle-based pattern is used from the C-side for the inspection and manipulation of the objects on the Rust side. This is a little clunky, but it does kind of do the job.

=Building=
The C function declarations are auto-generated using the `cbindgen` tool; so you will need to have that installed to build the thing.  If you don't have it installed, you can install it by doing:

```
cargo install cbindgen
```

Then it should be good to go.