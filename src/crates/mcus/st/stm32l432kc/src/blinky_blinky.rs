use core::arch::asm;

// TODO: these are temporary (but correct layout), purely for the blinking LED...
#[repr(C, packed)]
#[allow(non_snake_case)]
struct RccRegisters {
    CR: u32,
    ICSCR: u32,
    CFGR: u32,
    PLLCFGR: u32,
    PLLSAI1CFGR: u32,
    _reserved_14: u32,
    CIER: u32,
    CIFR: u32,
    CICR: u32,
    _reserved_24: u32,
    AHB1RSTR: u32,
    AHB2RSTR: u32,
    AHB3RSTR: u32,
    _reserved_34: u32,
    APB1RSTR1: u32,
    APB1RSTR2: u32,
    APB2RSTR: u32,
    _reserved_44: u32,
    AHB1ENR: u32,
    AHB2ENR: u32,
    AHB3ENR: u32,
    _reserved_54: u32,
    APB1ENR1: u32,
    APB1ENR2: u32,
    APB2ENR: u32,
    _reserved_64: u32,
    AHB1SMENR: u32,
    AHB2SMENR: u32,
    AHB3SMENR: u32,
    _reserved_74: u32,
    APB1SMENR1: u32,
    APB1SMENR2: u32,
    APB2SMENR: u32,
    _reserved_84: u32,
    CCIPR: u32,
    _reserved_8c: u32,
    BDCR: u32,
    CSR: u32,
    CRRCR: u32,
    CCIPR2: u32
}

const _: () = assert!(core::mem::size_of::<RccRegisters>() == 0xa0, "RCC register bank must be 160 bytes (40 words)");

#[repr(C, packed)]
#[allow(non_snake_case)]
struct GpioRegisters {
    MODER: u32,
    OTYPER: u32,
    OSPEEDR: u32,
    PUPDR: u32,
    IDR: u32,
    ODR: u32,
    BSRR: u32,
    LCKR: u32,
    AFRL: u32,
    AFRH: u32,
    BRR: u32
}

const _: () = assert!(core::mem::size_of::<GpioRegisters>() == 0x2c, "GPIO register bank must be 44 bytes (11 words)");

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

        let delay = 200000; // Debug builds now include some optimisations, but if not: if cfg!(debug_assertions) { 10000 } else { 200000 };
        loop {
            let blink = core::ptr::read_volatile(gpiob_odr);
            core::ptr::write_volatile(gpiob_odr, blink ^ (1 << 3));
            for _ in 0..delay {
                asm!("nop");
            }
        }
    }
}
