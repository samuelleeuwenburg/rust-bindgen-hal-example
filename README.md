# Rust bindgen C HAL example

This is an example project showing how you can wrap C HAL libraries using the Rust bindgen crate

## Running the code

Plug in the nucleo devboard using the onboard programmer usb connection

```
$ cd stm32f439
$ cargo run
```

You can also build the target and debug using gdb

First make sure you run openocd in a seperate terminal window

```
$ cd stm32f439
$ openocd
```

Then build and flash the code onto the chip

```
$ cd stm32f439
$ cargo build
$ arm-none-eabi-gdb -x ./openocd.gdb -q ./target/thumbv7em-none-eabihf/debug/stmtest
```
