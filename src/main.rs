#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use rtic::app;
use atsamd_hal;

#[app(device = atsamd_hal::pac)]
mod app {
    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut device = cx.device;

        // Get the clocks & tokens
        let (_buses, clocks, tokens) = atsamd_hal::clock::v2::clock_system_at_reset(
            device.OSCCTRL,
            device.OSC32KCTRL,
            device.GCLK,
            device.MCLK,
            &mut device.NVMCTRL,
        );

        // This is required because the `sercom` and `rtc` modules have not yet
        // been update to use `clock::v2`
        let (_, _, _, mut mclk) = unsafe { clocks.pac.steal() };

        // Get the pins
        let pins = atsamd_hal::gpio::Pins::new(device.PORT);

        // Take `Dfll` 48 MHz, divide down to `2 MHz` through `Gclk1`
        let (gclk2, dfll) = atsamd_hal::clock::v2::gclk::Gclk::new(tokens.gclks.gclk2, clocks.dfll);
        let gclk2 = gclk2.div(atsamd_hal::clock::v2::gclk::GclkDiv8::Div(24)).enable();

        // Output `Gclk1` on PB15 pin
        let (_gclk_out2, gclk2) = atsamd_hal::clock::v2::gclkio::GclkOut::enable(tokens.gclk_io.gclk_out2, pins.pb16, gclk2);

        pins.pa16.into_alternate::<atsamd_hal::gpio::M>();

        (Shared { }, Local { }, init::Monotonics())
    }
}
