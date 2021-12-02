#![no_main]
#![no_std]

mod cdc;
mod params;
mod setup;

use cortex_m::asm;
use panic_halt as _;

use cdc::{
    device::CdcDevice,
    inbound::{Reader, UsbInbound},
    outbound::{UsbOutbound, Writer},
};
use params::SM2MParams;
use stm32f4xx_hal::{
    gpio::{gpioa::PA0, gpioc::PC13, Input, Output, PinState, PullUp, PushPull},
    otg_fs,
};

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PC13<Output<PushPull>>,
        btn: PA0<Input<PullUp>>,
        usb: CdcDevice,
        params: SM2MParams,
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Setup MCU
        setup::core(&mut cx.core);
        let peripherals = setup::device(cx.device);

        // Configure on board peripherals
        let led = peripherals
            .gpioc
            .pc13
            .into_push_pull_output_in_state(PinState::High);
        let btn = peripherals.gpioa.pa0.into_pull_up_input();

        // Configure USB
        let usb_conf = otg_fs::USB {
            usb_global: peripherals.usb_global,
            usb_device: peripherals.usb_device,
            usb_pwrclk: peripherals.usb_pwrclk,
            pin_dm: peripherals.gpioa.pa11.into_alternate(),
            pin_dp: peripherals.gpioa.pa12.into_alternate(),
            hclk: peripherals.clocks.hclk(),
        };

        init::LateResources {
            led,
            btn,
            usb: CdcDevice::new(usb_conf),
            params: Default::default(),
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(capacity = 5, resources = [usb])]
    fn usb_inbound(cx: usb_inbound::Context, inbound: UsbInbound) {
        let mut usb_device = cx.resources.usb;
        match inbound {
            UsbInbound::GetVersion => {
                let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
                let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
                let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
                let outbound = UsbOutbound::Version(major, minor, patch);
                usb_device.lock(|usb_device| {
                    usb_device.write_ex(outbound).ok();
                });
            }
        };
    }

    #[task(priority = 2, binds = OTG_FS, spawn = [usb_inbound], resources = [usb])]
    fn usb_global(cx: usb_global::Context) {
        read_usb_packet(cx.resources.usb).and_then(|request| cx.spawn.usb_inbound(request).ok());
    }

    #[task(priority = 2, binds = OTG_FS_WKUP, spawn = [usb_inbound], resources = [usb])]
    fn usb_wkup(cx: usb_wkup::Context) {
        read_usb_packet(cx.resources.usb).and_then(|request| cx.spawn.usb_inbound(request).ok());
    }

    extern "C" {
        fn TAMP_STAMP();
    }
};

fn read_usb_packet(usb: &mut CdcDevice) -> Option<UsbInbound> {
    if usb.poll() {
        usb.read_ex().unwrap_or(None)
    } else {
        None
    }
}
