# ascii-tactics

A stupid small experiment in building a simulation with a retro ASCII-style interface, powered by Rust and egui.

Just something to learn, explore, and have fun with.

## Requirements

- Rust (stable)
- [cargo-audit](https://github.com/rustsec/rustsec) *(optional, for security checks)*
- [cross](https://github.com/cross-rs/cross) *(optional, for cross-compilation)*

## Usage

### Build and run locally

```bash
make build      # build native release
make run        # run debug build

Format, lint, and check

make fmt        # rustfmt
make lint       # clippy (deny warnings)
make audit      # check for vulnerable dependencies

(Optional) Cross-compile

make build-windows
make build-linux
```

## License
MIT – see LICENSE.
