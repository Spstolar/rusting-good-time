# General Steps
 783  rustup target add thumbv7em-none-eabihf
 1784  cargo install cargo-binutils
 1786  rustup component add llvm-tools-preview

# Running an Example
 1790  git clone https://github.com/mtthw-meyer/daisy-looper.git
 1791  cd daisy-looper/
 807  cargo build --release
 1809  cargo objcopy --release -- -O binary daisy-looper.bin
 1811  dfu-util --list
 1812  dfu-util -a 0 -s 0x08000000:leave -D daisy-looper.bin 

# Better Example Run
 1815  git clone https://github.com/mtthw-meyer/libdaisy-rust.git
 1817  cd libdaisy-rust/
 1819  cargo objcopy --example blinky --release -- -O binary blinky.bin
 1821  dfu-util -a 0 -s 0x08000000:leave -D blinky.bin 
