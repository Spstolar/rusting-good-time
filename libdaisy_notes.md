* Hardware target

> `rustup target add thumbv7em-none-eabihf`

This is adding a cross compilation target for ARM Cortex-M4F and Cortex-M7F (with FPU support). See [this crate](https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/). Some info about `rustup target add` is [here](https://rust-lang.github.io/rustup/cross-compilation.html).

* cargo-binutils

> `cargo install cargo-binutils`

[github for the project](https://github.com/rust-embedded/cargo-binutils). The `cargo install` command is explained [here](https://doc.rust-lang.org/cargo/commands/cargo-install.html).

> `rustup component add llvm-tools-preview`

This is another part of the install. Components are explained [here](https://rust-lang.github.io/rustup/concepts/components.html).
