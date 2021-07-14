
Because we write code on our local computer and it has a different architecture than the ARM Cortex-M, we have to cross-compile. Using `rustup` ([an installer for Rust](https://rustup.rs/)), we grab the appropriate target because the Seed falls under Cortex-M4F and M7F with hardware floating point (ARMv7E-M architecture) we use:

```
rustup target add thumbv7em-none-eabihf
```

We also install `cargo-binutils` in order to use Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain:

```
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Next, install `cargo-generate` so that you can easily grab a template for your project:

```
cargo install cargo-generate
```

Finally (at least for setup), you will use `cargo-generate` to grab a template:

```
cargo generate --git https://github.com/githubusername/mytemplate.git
```

# Memory.x

http://m68hc11.serveftp.org/memory_x-1.php

The memory.x file is used when you use the m68hc11elfb linker script. It defines memory banks that tell the linker where to put the text, data and other program sections. The name of memory banks are specified within the linker script. A typical memory.x file defines the MEMORY banks as follows:

```
MEMORY
{
  page0 (rwx) : ORIGIN = 0x0, LENGTH = 256
  text  (rx)  : ORIGIN = 0xE000, LENGTH = 2048
  data        : ORIGIN = 0x0, LENGTH = 0
}
```

# cargo-generate ideas

use https://github.com/rust-embedded/cortex-m-quickstart for good, relevant example

# Terms

* Serial
* UART - Universal Asynchronous Receiver Transmitter
* RS-232 - also EIA-232, the data format
* SPI - Serial Peripheral Interface
* I2C - Inter-integrated Circuit (I-squared-C)
* TTL - Transistor Transistor Logic
* I/O - input/output
* CPU - Central Processing Unit
* Memory-mapped I/O
* Cross compilation - when the source code compiler is targeting a different operating system than the one it is currently hosted in.
* ELF - Executable and Linkable Format
