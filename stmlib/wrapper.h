#define STM32F439xx

#include "stm32f439xx.h"
#include "stm32f4xx.h"
#include "system_stm32f4xx.h"

#include "cmsis_compiler.h"
#include "cmsis_gcc.h"
#include "cmsis_version.h"
#include "core_cm4.h"
#include "mpu_armv7.h"

#include "stm32f4xx_hal_conf.h"
#include "stm32f4xx_it.h"

#include "Legacy/stm32_hal_legacy.h"
#include "stm32f4xx_hal.h"
#include "stm32f4xx_hal_adc.h"
#include "stm32f4xx_hal_adc_ex.h"
#include "stm32f4xx_hal_cortex.h"
#include "stm32f4xx_hal_def.h"
#include "stm32f4xx_hal_dma.h"
#include "stm32f4xx_hal_dma_ex.h"
#include "stm32f4xx_hal_exti.h"
#include "stm32f4xx_hal_flash.h"
#include "stm32f4xx_hal_flash_ex.h"
#include "stm32f4xx_hal_flash_ramfunc.h"
#include "stm32f4xx_hal_gpio.h"
#include "stm32f4xx_hal_gpio_ex.h"
#include "stm32f4xx_hal_pwr.h"
#include "stm32f4xx_hal_pwr_ex.h"
#include "stm32f4xx_hal_rcc.h"
#include "stm32f4xx_hal_rcc_ex.h"
#include "stm32f4xx_hal_uart.h"