#![no_std]
#![no_main]
#![feature(future_join)]

use core::future::join;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use cortex_m_rt::exception;
use embassy_futures::yield_now;
use mm32g0001_pac as pac;
use panic_halt as _;
use rtt_target::rtt_init_defmt;

static TICK: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    let c = TICK.load(Ordering::Relaxed) + 1;
    TICK.store(c, Ordering::Relaxed);
}

defmt::timestamp!("{=u32:ms}", get_tick());

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_defmt!(rtt_target::ChannelMode::NoBlockSkip, 64);
    defmt::info!("program start");

    let p = unsafe { pac::Peripherals::steal() };
    // set flash latency
    let flash = p.flash;
    flash.acr().modify(|_, w| unsafe { w.latency().bits(1) });

    // enable HSI (48MHz) and set as system clock source
    let rcc = p.rcc;
    rcc.cr().modify(|_, w| w.hsion().set_bit());
    // wait until HSI is ready
    while rcc.cr().read().hsion().bit_is_clear() {}
    // use HSI as sysclk
    rcc.cfgr().modify(|_, w| unsafe { w.sw().bits(0b10) });

    // setup systick
    let cp = pac::CorePeripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    syst.set_reload(48_000 - 1); // 1ms
    syst.enable_counter();
    syst.enable_interrupt();

    // enable GPIOA
    rcc.ahbenr().modify(|_, w| w.gpioa().set_bit());

    // set pin mode
    let gpioa = p.gpioa;
    gpioa.crh().write(|w| unsafe {
        w.mode11().bits(0b10).cnf11().bits(0);
        w.mode10().bits(0b10).cnf10().bits(0)
    });

    let blink_led = async {
        loop {
            gpioa.bsrr().write(|w| w.br11().set_bit());
            defmt::info!("on");
            delay_ms(1000).await;

            gpioa.bsrr().write(|w| w.bs11().set_bit());
            defmt::info!("off");
            delay_ms(1000).await;
        }
    };

    let toggle_pin = async {
        loop {
            gpioa.bsrr().write(|w| w.br10().set_bit());
            delay_ms(2).await;

            gpioa.bsrr().write(|w| w.bs10().set_bit());
            delay_ms(2).await;
        }
    };

    let all_tasks = join!(blink_led, toggle_pin);

    embassy_futures::block_on(all_tasks).0
}

fn get_tick() -> u32 {
    TICK.load(Ordering::Relaxed)
}

async fn delay_ms(n: u32) {
    let target = get_tick() + n;
    while get_tick() < target {
        yield_now().await;
    }
}
