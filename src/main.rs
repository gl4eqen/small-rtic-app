#![no_main]
#![no_std]

use panic_semihosting as _; // panic handler
use rtic::app;

#[app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::debug;
    use cortex_m_semihosting::hprintln;

    #[shared]
    struct Shared {
        big: Option<[u8; 2000]>
    }

    #[local]
    struct Local {
        big: Option<[u8; 1000]>
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        let a = [0_u8; 66666];
        hprintln!("Init!").unwrap();
        (Shared { big: None }, Local { big: None }, init::Monotonics())
    }

    #[idle(local = [ big ], shared = [ big ])]
    fn idle(_: idle::Context) -> ! {
        hprintln!("Idle!").unwrap();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        unreachable!();
    }
}
