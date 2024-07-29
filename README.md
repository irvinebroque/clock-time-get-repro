

### Building

1. `cargo build --release`
2. `rustup target add wasm32-wasi` (if you don't have installed)
3. `cargo build --target wasm32-wasi --release`
4. `npx wrangler@wasm dev target/wasm32-wasi/release/hello_world.wasm` (run in remote dev)

### Error

```
error[E0432]: unresolved import `wasi::wasi_unstable`
 --> src/main.rs:2:11
  |
2 | use wasi::wasi_unstable::{clock_time_get, Clockid, Timestamp};
  |           ^^^^^^^^^^^^^ could not find `wasi_unstable` in `wasi`
```