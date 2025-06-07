cfg_if::cfg_if! {
    if #[cfg(any(feature = "smeg-mcu-arm-cortex_m4_family"))] {
        mod cortex_m4_family;
        pub use cortex_m4_family::{collect_isr_vectors, IsrVectorTableBuilder};
    }
}
