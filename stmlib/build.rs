extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let mut cfg = cc::Build::new();

    // disable warnings produced by the lib
    cfg.warnings(false);

    cfg.define("STM32F439xx", None)
        .define("USE_HAL_DRIVER", None)
        .include("c/Core/Inc")
        .include("c/Drivers/STM32F4xx_HAL_Driver/Inc")
        .include("c/Drivers/STM32F4xx_HAL_Driver/Inc/Legacy")
        .include("c/Drivers/CMSIS/Device/ST/STM32F4xx/Include")
        .include("c/Drivers/CMSIS/Include")
        .file("c/Core/Src/stm32f4xx_it.c")
        .file("c/Core/Src/stm32f4xx_hal_msp.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_rcc.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_rcc_ex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash_ex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_flash_ramfunc.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_gpio.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_dma_ex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_dma.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pwr.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pwr_ex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_cortex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_exti.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_eth.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_tim.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_tim_ex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_uart.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pcd.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_hal_pcd_ex.c")
        .file("c/Drivers/STM32F4xx_HAL_Driver/Src/stm32f4xx_ll_usb.c")
        .file("c/Core/Src/system_stm32f4xx.c")
        .out_dir(dst.join("lib"))
        .compile("stm32_hal");

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=static=stm32_hal");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./c/Core/Inc")
        .clang_arg("-I./c/Drivers/STM32F4xx_HAL_Driver/Inc")
        .clang_arg("-I./c/Drivers/STM32F4xx_HAL_Driver/Inc/Legacy")
        .clang_arg("-I./c/Drivers/CMSIS/Device/ST/STM32F4xx/Include")
        .clang_arg("-I./c/Drivers/CMSIS/Include")
        .use_core()
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
