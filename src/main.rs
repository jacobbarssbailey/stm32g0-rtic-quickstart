#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32g0xx_hal as hal;

use panic_probe as _;
use rtic::app;

use hal::prelude::*;

#[app(device = hal::stm32, peripherals = true, dispatchers = [TIM16])]
mod app {

    use super::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        ctx.device.RCC.ahbenr.write(|w| w.dmaen().set_bit());
        rtt_init_print!();
        rprintln!("good morning!");

        let mut rcc = ctx.device.RCC.constrain();
        let _gpioa = ctx.device.GPIOA.split(&mut rcc);

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            continue;
        }
    }
}
