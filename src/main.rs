#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod logging;
use rtic::app;

#[app(device = microbit::pac, peripherals = true)]
mod app {
    use crate::logging;
    // NOTE: The defmt version of these macros will log the panic message using defmt
    // and then call core::panic!, so the rtt message will be emitted before panic is invoked
    #[cfg(feature = "use_defmt")]
    use panic_halt as _;

    #[cfg(feature = "use_rtt")]
    use rtt_target::{rtt_init_print};

    #[cfg(feature = "use_rtt")]
    use panic_rtt_target as _;

    use microbit::hal::prelude::*;

    #[shared]
    struct Shared { }

    #[local]
    struct Local { }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Initialize RTT if the feature is enabled
        #[cfg(feature = "use_rtt")]
        rtt_init_print!();

        logging::test_print("in init");

        (
            Shared { },
            Local {}
        )
    }
}
