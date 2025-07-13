#![no_std]
#![no_main]

use cortex_m as _;
use defmt::info;
use mm32g0001_pac as _;
use panic_halt as _;
use rtt_target::rtt_init_defmt;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_defmt!(rtt_target::ChannelMode::NoBlockSkip, 64);

    info!("Hello, World!");
    loop {}
}
