# `nvim-wrapper`

A simple wrapper which unset `LC_CTYPE` before running `nvim`/`nvim-qt`. See [neovim/neovim#9405](https://github.com/neovim/neovim/issues/9405) for more informations.

You'll need `rust` (version 1.31 minimum) to install it.

With cargo:
```
cargo install --git https://github.com/justinrlle/nvim-wrapper
```

Alternatively, just download the [main.rs](./src/main.rs) file, and compile it with `rustc`:
```
rustc --edition=2018 -C opt-level=3 src/main.rs -o nvim-wrapper
```
and then put `nvim-wrapper.exe` in a folder in your path.
