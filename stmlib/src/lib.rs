#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn toggle_pin(gpio: usize, pin: usize) {
    unsafe {
        HAL_GPIO_TogglePin(gpio as *mut GPIO_TypeDef, pin as u16);
    }
}

pub fn hal_init() {
    unsafe {
        HAL_Init();
    }
}

pub fn led_init(gpio: usize, pin: usize) {
    // set defaults for a digital output pin
    let mut pin = GPIO_InitTypeDef {
        Pin: pin as u32,
        Mode: MODE_OUTPUT,
        Pull: GPIO_NOPULL,
        Speed: GPIO_SPEED_FREQ_VERY_HIGH,
        Alternate: 0,
    };

    unsafe {
        HAL_GPIO_Init(gpio as *mut GPIO_TypeDef, &mut pin);
    }
}

pub fn clock_enable() {
    // enable the GPIOB clock
    let rcc = RCC_BASE as *mut RCC_TypeDef;
    unsafe { (*rcc).AHB1ENR |= RCC_AHB1ENR_GPIOBEN };
}
