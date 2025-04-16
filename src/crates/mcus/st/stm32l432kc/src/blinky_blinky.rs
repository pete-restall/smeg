use core::arch::asm;

// TODO: these are temporary (but correct layout), purely for the blinking LED...
#[repr(C)]
#[allow(non_snake_case)]
struct RccRegisters {
    CR: usize,
    ICSCR: usize,
    CFGR: usize,
    PLLCFGR: usize,
    PLLSAI1CFGR: usize,
    _reserved_14: usize,
    CIER: usize,
    CIFR: usize,
    CICR: usize,
    _reserved_24: usize,
    AHB1RSTR: usize,
    AHB2RSTR: usize,
    AHB3RSTR: usize,
    _reserved_34: usize,
    APB1RSTR1: usize,
    APB1RSTR2: usize,
    APB2RSTR: usize,
    _reserved_44: usize,
    AHB1ENR: usize,
    AHB2ENR: usize,
    AHB3ENR: usize,
    _reserved_54: usize,
    APB1ENR1: usize,
    APB1ENR2: usize,
    APB2ENR: usize,
    _reserved_64: usize,
    AHB1SMENR: usize,
    AHB2SMENR: usize,
    AHB3SMENR: usize,
    _reserved_74: usize,
    APB1SMENR1: usize,
    APB1SMENR2: usize,
    APB2SMENR: usize,
    _reserved_84: usize,
    CCIPR: usize,
    _reserved_8c: usize,
    BDCR: usize,
    CSR: usize,
    CRRCR: usize,
    CCIPR2: usize
}

const _: () = if core::mem::size_of::<RccRegisters>() != 0xa0 { panic!("RCC register bank must be 160 bytes (40 words)"); };

#[repr(C)]
#[allow(non_snake_case)]
struct GpioRegisters {
    MODER: usize,
    OTYPER: usize,
    OSPEEDR: usize,
    PUPDR: usize,
    IDR: usize,
    ODR: usize,
    BSRR: usize,
    LCKR: usize,
    AFRL: usize,
    AFRH: usize,
    BRR: usize
}

const _: () = if core::mem::size_of::<GpioRegisters>() != 0x2c { panic!("GPIO register bank must be 44 bytes (11 words)"); };

unsafe extern "C" {
    // TODO: uppercase all of the statics as per convention...(before committing to git)
    unsafe static mut __LINKER_PERIPHERALS_AHB1_RCC: RccRegisters;
    unsafe static mut __LINKER_PERIPHERALS_AHB2_GPIOB: GpioRegisters;
}

// TODO: just a blinky-blinky to make sure the code links and runs properly on the Nucleo board.
#[allow(static_mut_refs)]
pub unsafe fn blinky_blinky() -> ! {
    // TODO: verify that the linker script gets the addresses right - because LLD differs from LD in some really horrible, subtle ways...
    if !core::ptr::addr_eq(&raw const __LINKER_PERIPHERALS_AHB1_RCC, 0x40021000 as *const u32) {
        panic!("Linker script for AHB1 RCC register is wrong");
    }

    if !core::ptr::addr_eq(&raw const __LINKER_PERIPHERALS_AHB2_GPIOB, 0x48000400 as *const u32) {
        panic!("Linker script for AHB2 GPIOB register is wrong");
    }

    unsafe {
        // Enable clocking of the GPIOB module
        let rcc_ahb2enr = &raw mut __LINKER_PERIPHERALS_AHB1_RCC.AHB2ENR;
        core::ptr::write_volatile(rcc_ahb2enr, 2);

        asm!(r#"
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
        "#);

        // Set LED pin to output
        let gpio_b_moder = &raw mut __LINKER_PERIPHERALS_AHB2_GPIOB.MODER;
        core::ptr::write_volatile(gpio_b_moder, 0xfffffe7f);

        // Port output register
        let gpiob_odr = &raw mut __LINKER_PERIPHERALS_AHB2_GPIOB.ODR;

        let delay = if cfg!(debug_assertions) { 10000 } else { 200000 };
        loop {
            let blink = core::ptr::read_volatile(gpiob_odr);
            core::ptr::write_volatile(gpiob_odr, blink ^ (1 << 3));
            for _ in 0..delay {
                asm!("nop");
            }
        }
    }
}
