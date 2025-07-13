#![no_std]
#![no_main]
#![feature(future_join)]

use mm32_pac as pac;
use panic_halt as _;
use rtt_target::rtt_init_defmt;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_defmt!(rtt_target::ChannelMode::NoBlockSkip, 64);
    defmt::info!("program start");

    let p = unsafe { pac::Peripherals::steal() };

    // set flash latency
    let flash = p.flash;
    flash.acr().modify(|_, w| unsafe { w.latency().bits(1) });

    // enable HSI and set as system clock source
    let rcc = p.rcc;
    rcc.cr().modify(|_, w| w.hsion().set_bit());
    rcc.cfgr().modify(|_, w| unsafe { w.sw().bits(0b10) });

    // enable GPIOA
    rcc.ahbenr().modify(|_, w| w.gpioa().set_bit());

    // set pin mode
    let gpioa = p.gpioa;
    gpioa.crh().write(|w| unsafe {
        w.mode11().bits(0b10).cnf11().bits(0);
        w.mode10().bits(0b10).cnf10().bits(0)
    });

    loop {
        gpioa.bsrr().write(|w| w.br11().set_bit());
        defmt::info!("on");
        delay();

        gpioa.bsrr().write(|w| w.bs11().set_bit());
        defmt::info!("off");
        delay();
    }
}

fn delay() {
    for _ in 0..500_000 {
        cortex_m::asm::nop();
    }
}
