 1863  git checkout xnor/field 
 1864  cargo objcopy --example field --release -- -O binary field.bin
 1866  dfu-util -a 0 -s 0x08000000:leave -D field.bin
