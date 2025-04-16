/* TODO: A small RAM section (< 32 bytes ?) can be carved out for the kernel that is not initialised like BSS, so that software resets can persist state (ie. unhandled exceptions or NMIs) across resets for debugging or remedial action */
FLASH_SIZE_BYTES = 256K;
SRAM1_SIZE_BYTES = 48K;
SRAM2_SIZE_BYTES = 16K;
BSRAM_SIZE_BYTES = 8K;
KERNEL_STACK_SIZE_WORDS = 1;

ENTRY(_resetHandler);

MEMORY {
	FLASH_INITIAL_SP (r) : ORIGIN = 0x08000000, LENGTH = 4

	FLASH_ISR_VECTORS (r) :
		ORIGIN = ORIGIN(FLASH_INITIAL_SP) + LENGTH(FLASH_INITIAL_SP),
		LENGTH = (15 + 85) * 4

	FLASH_TEXT (rx) :
		ORIGIN = ORIGIN(FLASH_ISR_VECTORS) + LENGTH(FLASH_ISR_VECTORS),
		LENGTH = FLASH_SIZE_BYTES - (ORIGIN(FLASH_ISR_VECTORS) + LENGTH(FLASH_ISR_VECTORS))

	SRAM (rwx) : ORIGIN = 0x20000000, LENGTH = SRAM1_SIZE_BYTES + SRAM2_SIZE_BYTES
}

SECTIONS {
	.code.boot 0x00000000 /*(READONLY)*/ : {
		__linker_code_boot_start = .;
		. += 0x40000;
		__linker_code_boot_past_end = .;
	}

	.code.reserved.0 0x00040000 : { . += ORIGIN(FLASH_INITIAL_SP) - .; }

	.code.flash.kernel.isr_vector_table /*(READONLY)*/ : {
		KEEP(* (.isr_vector_table.arm))
		KEEP(* (.isr_vector_table.mcu))
		ASSERT(
			(. - ADDR(.code.flash.kernel.isr_vector_table) == LENGTH(FLASH_ISR_VECTORS)),
			"The interrupt vector table must contain 15 ARM-defined vectors and 85 STM32L432KC-specific vectors");
	} > FLASH_ISR_VECTORS

	.code.reserved.1 0x08040000 : { . += 0x7fc0000; }

	.code.sram2 0x10000000 : {
		. += 0x4000;
	}

	.code.reserved.2 0x10004000 : { . += 0xffec000; }

	.code.system_memory 0x1fff0000 : { __linker_code_system_memory = .; . += 0x7000; }
	.code.system_memory 0x1fff7000 /*(READONLY)*/ : { __linker_code_otp = .; . += 0x400; }
	.code.reserved.2 0x1fff7400 : { . += 0x400; }
	.code.options 0x1fff7800 /*(READONLY)*/ : { __linker_code_options = .; . += 0x10; }
	.code.reserved.2 0x1fff7810 : { . += 0x87f0; }

	.sram.stack.kernel (ORIGIN(SRAM) + SRAM1_SIZE_BYTES) : {
		. = ALIGN(8);
		*: (.kernel.tasks.stacks.default* .kernel.tasks.stacks.default.*)
		*: (.kernel.tasks.stacks.isrs* .kernel.tasks.stacks.isrs.*)
		__linker_sram_stack_kernel_bottom = .;
		. += (KERNEL_STACK_SIZE_WORDS - 1) * 4;
		__linker_sram_stack_kernel_top = .;
		. += 4;
		__linker_sram_stack_kernel_pastTop = .;
	} > SRAM

	.sram.data.retained16k.kernel : {
		. = ALIGN(4);
		__linker_sram_data_retained16k_kernel_start = .;
		*: (.data.retained16k .data.retained16k*)
		. = ALIGN(4);
		__linker_sram_data_retained16k_kernel_past_end = .;
	} > SRAM AT> FLASH_TEXT
	__linker_sram_data_retained16k_kernel_lma_start = ALIGN(LOADADDR(.sram.data.retained16k.kernel), 4);
	__linker_sram_data_retained16k_kernel_lma_past_end = __linker_sram_data_retained16k_kernel_lma_start + (__linker_sram_data_retained16k_kernel_past_end - __linker_sram_data_retained16k_kernel_start);

	.sram.bss.retained16k.kernel : {
		. = ALIGN(4);
		__linker_sram_bss_retained16k_kernel_start = .;
		*: (.bss.retained16k .bss.retained16k*)
		. = ALIGN(4);
		__linker_sram_bss_retained16k_kernel_past_end = .;
	} > SRAM

	.sram.data.retained16k.app : {
		. = ALIGN(4);
		__linker_sram_data_retained16k_app_start = .;
		* (.data.retained16k .data.retained16k*)
		. = ALIGN(4);
		__linker_sram_data_retained16k_app_past_end = .;
	} > SRAM AT> FLASH_TEXT
	__linker_sram_data_retained16k_app_lma_start = ALIGN(LOADADDR(.sram.data.retained16k.app), 4);
	__linker_sram_data_retained16k_app_lma_past_end = __linker_sram_data_retained16k_app_lma_start + (__linker_sram_data_retained16k_app_past_end - __linker_sram_data_retained16k_app_start);

	.sram.bss.retained16k.app : {
		. = ALIGN(4);
		__linker_sram_bss_retained16k_app_start = .;
		* (.bss.retained16k .bss.retained16k*)
		. = ALIGN(4);
		__linker_sram_bss_retained16k_app_past_end = .;

		ASSERT(
			(__linker_sram_bss_retained16k_app_past_end <= ORIGIN(SRAM) + SRAM1_SIZE_BYTES + 16K),
			"The total sizeof(.data.retained16k.*) + sizeof(.bss.retained16k.*) is greater than 16KiB and so exceeds the amount of retained SRAM2; if the entire SRAM is to be retained then use of these sections is un-necessary");
	} > SRAM

	.sram.data.kernel ORIGIN(SRAM) : {
		. = ALIGN(4);
		__linker_sram_data_kernel_start = .;
		*: (.data .data*)
		. = ALIGN(4);
		__linker_sram_data_kernel_past_end = .;
	} > SRAM AT> FLASH_TEXT
	__linker_sram_data_kernel_lma_start = ALIGN(LOADADDR(.sram.data.kernel), 4);
	__linker_sram_data_kernel_lma_past_end = __linker_sram_data_kernel_lma_start + (__linker_sram_data_kernel_past_end - __linker_sram_data_kernel_start);

	.sram.bss.kernel : {
		. = ALIGN(4);
		__linker_sram_bss_kernel_start = .;
		*: (.bss .bss*)
		. = ALIGN(4);
		__linker_sram_bss_kernel_past_end = .;
	} > SRAM

	.sram.data.app : {
		. = ALIGN(4);
		__linker_sram_data_app_start = .;
		* (.data .data*)
		. = ALIGN(4);
		__linker_sram_data_app_past_end = .;
	} > SRAM AT> FLASH_TEXT
	__linker_sram_data_app_lma_start = ALIGN(LOADADDR(.sram.data.app), 4);
	__linker_sram_data_app_lma_past_end = __linker_sram_data_app_lma_start + (__linker_sram_data_app_past_end - __linker_sram_data_app_start);

	.sram.bss.app : {
		. = ALIGN(8);
		__linker_sram_bss_app_start = .;
		* (.app.tasks.stacks.default* .app.tasks.stacks.default.*)
		* (.bss .bss*)
		. = ALIGN(4);
		__linker_sram_bss_app_past_end = .;
	} > SRAM

	.code.flash.kernel.init_array /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_kernel_init_array_start = .;
		KEEP(* (.init_array))
		KEEP(* (SORT_BY_INIT_PRIORITY(.init_array*)))
		__linker_code_flash_kernel_init_array_past_end = .;
	} > FLASH_TEXT

	.code.flash.kernel.fini_array /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_kernel_fini_array_start = .;
		KEEP(*: (.fini_array))
		KEEP(*: (SORT_BY_INIT_PRIORITY(.fini_array*)))
		__linker_code_flash_kernel_fini_array_past_end = .;
	} > FLASH_TEXT

	.code.flash.kernel.rodata /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_kernel_rodata_start = .;
		*: (.rodata .rodata*)
		__linker_code_flash_kernel_rodata_past_end = .;
	} > FLASH_TEXT

	.code.flash.kernel.text /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_kernel_text_start = .;
		*: (.text .text*)
		__linker_code_flash_kernel_text_past_end = .;
	} > FLASH_TEXT

	.code.flash.app.init_array /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_app_init_array_start = .;
		KEEP(* (.init_array))
		KEEP(* (SORT_BY_INIT_PRIORITY(.init_array*)))
		__linker_code_flash_app_init_array_past_end = .;
	} > FLASH_TEXT

	.code.flash.app.fini_array /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_app_fini_array_start = .;
		KEEP(* (.fini_array))
		KEEP(* (SORT_BY_INIT_PRIORITY(.fini_array*)))
		__linker_code_flash_app_fini_array_past_end = .;

		ASSERT(
			(. - __linker_code_flash_app_fini_array_start == 0),
			"Finalisers are not supported / will not be executed by the firmware");
	} > FLASH_TEXT

	.code.flash.app.rodata /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_app_rodata_start = .;
		* (.rodata .rodata*)
		__linker_code_flash_app_rodata_past_end = .;
	} > FLASH_TEXT

	.code.flash.app.text /*(READONLY)*/ : {
		. = ALIGN(4);
		__linker_code_flash_app_text_start = .;
		* (.text .text*)
		__linker_code_flash_app_text_past_end = .;
	} > FLASH_TEXT

	.code.flash.initial_stack_pointer /*(READONLY)*/ : {
		__linker_code_flash_initialStackPointer = .;
		LONG(__linker_sram_stack_kernel_top);
	} > FLASH_INITIAL_SP

	.undefined.0 0x20010000 : { . += 0x1fff0000; }

	.peripherals.apb1.tim2 0x40000000 : { __linker_peripherals_apb1_tim2 = .; . += 0x400; }
	.peripherals.apb1.reserved.0 0x40000400 : { . += 0xc00; }
	.peripherals.apb1.tim6 0x40001000 : { __linker_peripherals_apb1_tim6 = .; . += 0x400; }
	.peripherals.apb1.tim7 0x40001400 : { __linker_peripherals_apb1_tim7 = .; . += 0x400; }
	.peripherals.apb1.reserved.1 0x40001800 : { . += 0xf00; }
	.peripherals.apb1.rtc 0x40002800 : { __linker_peripherals_apb1_rtc = .; . += 0x400; }
	.peripherals.apb1.wwdg 0x40002c00 : { __linker_peripherals_apb1_wwdg = .; . += 0x400; }
	.peripherals.apb1.iwdg 0x40003000 : { __linker_peripherals_apb1_iwdg = .; . += 0x400; }
	.peripherals.apb1.reserved.2 0x40003400 : { . += 0x800; }
	.peripherals.apb1.spi3 0x40003c00 : { __linker_peripherals_apb1_spi3 = .; . += 0x400; }
	.peripherals.apb1.reserved.3 0x40004000 : { . += 0x400; }
	.peripherals.apb1.usart2 0x40004400 : { __linker_peripherals_apb1_usart2 = .; . += 0x400; }
	.peripherals.apb1.reserved.4 0x40004800 : { . += 0xc00; }
	.peripherals.apb1.i2c1 0x40005400 : { __linker_peripherals_apb1_i2c1 = .; . += 0x400; }
	.peripherals.apb1.reserved.5 0x40005800 : { . += 0x400; }
	.peripherals.apb1.i2c3 0x40005c00 : { __linker_peripherals_apb1_i2c3 = .; . += 0x400; }
	.peripherals.apb1.crs 0x40006000 : { __linker_peripherals_apb1_crs = .; . += 0x400; }
	.peripherals.apb1.can1 0x40006400 : { __linker_peripherals_apb1_can1 = .; . += 0x400; }
	.peripherals.apb1.usb.fs 0x40006800 : { __linker_peripherals_apb1_usb_fs = .; . += 0x400; }
	.peripherals.apb1.usb.sram 0x40006c00 : { __linker_peripherals_apb1_usb_sram = .; . += 0x400; }
	.peripherals.apb1.pwr 0x40007000 : { __linker_peripherals_apb1_pwr = .; . += 0x400; }
	.peripherals.apb1.dac1 0x40007400 : { __linker_peripherals_apb1_dac1 = .; . += 0x400; }
	.peripherals.apb1.opamp 0x40007800 : { __linker_peripherals_apb1_opamp = .; . += 0x400; }
	.peripherals.apb1.lptim1 0x40007c00 : { __linker_peripherals_apb1_lptim1 = .; . += 0x400; }
	.peripherals.apb1.lpuart1 0x40008000 : { __linker_peripherals_apb1_lpuart1 = .; . += 0x400; }
	.peripherals.apb1.reserved.6 0x40008400 : { . += 0x400; }
	.peripherals.apb1.swpmi1 0x40008800 : { __linker_peripherals_apb1_swpmi1 = .; . += 0x400; }
	.peripherals.apb1.reserved.7 0x40008c00 : { . += 0x800; }
	.peripherals.apb1.lptim2 0x40009400 : { __linker_peripherals_apb1_lptim2 = .; . += 0x400; }
	.peripherals.apb1.reserved.9 0x40009800 : { . += 0x6800; }

	.peripherals.apb2.syscfg 0x40010000 : { __linker_peripherals_apb2_syscfg = .; . += 0x30; }
	.peripherals.apb2.reserved.0 0x40010030 : { . += 0x1d0; }
	.peripherals.apb2.comp 0x40010200 : { __linker_peripherals_apb2_comp = .; . += 0x200; }
	.peripherals.apb2.exti 0x40010400 : { __linker_peripherals_apb2_exti = .; . += 0x400; }
	.peripherals.apb2.reserved.1 0x40010800 : { . += 0x1400; }
	.peripherals.apb2.firewall 0x40011c00 : { __linker_peripherals_apb2_firewall = .; . += 0x400; }
	.peripherals.apb2.reserved.2 0x40012000 : { . += 0xc00; }
	.peripherals.apb2.tim1 0x40012c00 : { __linker_peripherals_apb2_tim1 = .; . += 0x400; }
	.peripherals.apb2.spi1 0x40013000 : { __linker_peripherals_apb2_spi1 = .; . += 0x400; }
	.peripherals.apb2.reserved.3 0x40013400 : { . += 0x400; }
	.peripherals.apb2.usart1 0x40013800 : { __linker_peripherals_apb2_usart1 = .; . += 0x400; }
	.peripherals.apb2.reserved.4 0x40013c00 : { . += 0x400; }
	.peripherals.apb2.tim15 0x40014000 : { __linker_peripherals_apb2_tim15 = .; . += 0x400; }
	.peripherals.apb2.tim16 0x40014400 : { __linker_peripherals_apb2_tim16 = .; . += 0x400; }
	.peripherals.apb2.reserved.5 0x40014800 : { . += 0xc00; }
	.peripherals.apb2.sai1 0x40015400 : { __linker_peripherals_apb2_sai1 = .; . += 0x400; }
	.peripherals.apb2.reserved.6 0x40015800 : { . += 0xa800; }

	.peripherals.ahb1.dma1 0x40020000 : { __linker_peripherals_ahb1_dma1 = .; . += 0x400; }
	.peripherals.ahb1.dma2 0x40020400 : { __linker_peripherals_ahb1_dma2 = .; . += 0x400; }
	.peripherals.ahb1.reserved.0 0x40020800 : { . += 0x800; }
	.peripherals.ahb1.rcc 0x40021000 : { __LINKER_PERIPHERALS_AHB1_RCC = .; . += 0x400; }
	.peripherals.ahb1.reserved.1 0x40021400 : { . += 0xc00; }
	.peripherals.ahb1.flash_registers 0x40022000 : { __linker_peripherals_ahb1_flashRegisters = .; . += 0x400; }
	.peripherals.ahb1.reserved.2 0x40022400 : { . += 0xc00; }
	.peripherals.ahb1.crc 0x40023000 : { __linker_peripherals_ahb1_crc = .; . += 0x400; }
	.peripherals.ahb1.reserved.3 0x40023400 : { . += 0xc00; }
	.peripherals.ahb1.tsc 0x40024000 : { __linker_peripherals_ahb1_tsc = .; . += 0x400; }
	.peripherals.ahb1.reserved.4 0x40024400 : { . += 0x7fdbc00; }

	.peripherals.ahb2.gpio_a 0x48000000 : { __linker_peripherals_ahb2_gpioA = .; . += 0x400; }
	.peripherals.ahb2.gpio_b 0x48000400 : { __linker_peripherals_ahb2_gpioB = .; . += 0x400; }
	.peripherals.ahb2.gpio_c 0x48000800 : { __linker_peripherals_ahb2_gpioC = .; . += 0x400; }
	.peripherals.ahb2.reserved.0 0x48000c00 : { . += 0x1000; }
	.peripherals.ahb2.gpio_h 0x48001c00 : { __linker_peripherals_ahb2_gpioH = .; . += 0x400; }
	.peripherals.ahb2.reserved.1 0x48002000 : { . += 0x7ffe000; }
	.peripherals.ahb2.reserved.2 0x50000000 : { . += 0x40000; }
	.peripherals.ahb2.adc 0x50040000 : { __linker_peripherals_ahb2_adc = .; . += 0x400; }
	.peripherals.ahb2.reserved.3 0x50040400 : { . += 0x20400; }
	.peripherals.ahb2.rng 0x50060800 : { __linker_peripherals_ahb2_rng = .; . += 0x400; }
	.peripherals.ahb2.reserved.4 0x50060c00 : { . += 0xff9f400; }

	.undefined.1 0x60000000 : { . += 0x30000000; }

	.quad_spi.flash 0x90000000 : { __linker_quadSpi_flash = .; . += 0x10000000; }

	.undefined.2 0xa0000000 : { . += 0x1000; }

	.quad_spi.registers 0xa0001000 : { __linker_quadSpi_registers = .; . += 0x400; }
	.quad_spi.registers.reserved.0 0xa0001400 : { . += 0x1fffebff; }

	.undefined.3 0xc0000000 : { . += 0x20000000; }

	.system.reserved.0 0xe0000000 : { . += 0xe000; }
	.system.scs 0xe000e000 : { __linker_system_scs = .; . += 0x1000; }
	.system.reserved.1 0xe000f000 : { . += 0xf0000; }
	.system.rom_table 0xe00ff000 : { __linker_system_romTable = .; . += 0x1000; }
	.system.reserved.2 0xe0100000 : { . += 0x1fefffff; }

	.ARM.attributes 0 /*(READONLY)*/ : { KEEP(* (.ARM.attributes)) }
	.comment 0 /*(READONLY)*/ : { KEEP(* (.comment)) }
	.debug_abbrev 0 /*(READONLY)*/ : { KEEP(* (.debug_abbrev)) }
	.debug_rnglists 0 /*(READONLY)*/ : { KEEP(* (.debug_rnglists)) }
	.debug_aranges 0 /*(READONLY)*/ : { KEEP(* (.debug_aranges)) }
	.debug_frame 0 /*(READONLY)*/ : { KEEP(* (.debug_frame)) }
	.debug_info 0 /*(READONLY)*/ : { KEEP(* (.debug_info)) }
	.debug_line 0 /*(READONLY)*/ : { KEEP(* (.debug_line)) }
	.debug_line_str 0 /*(READONLY)*/ : { KEEP(* (.debug_line_str)) }
	.debug_str 0 /*(READONLY)*/ : { KEEP(* (.debug_str)) }

	.unsupported.pic 0 : {
		* (.got* .plt*)
		ASSERT(
			SIZEOF(.unsupported.pic) == 0,
			"Turn off Position-Independent Code generation; some types of relocations are not supported / do not work correctly and will silently fail.  This produces subtly incorrect binaries");
	}

	/DISCARD/ : {
		* (.discard*)
		* (.ARM.exidx .ARM.exidx*)
		* (.ARM.extab .ARM.extab*)
	}
}
