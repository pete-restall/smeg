cfg_if::cfg_if! {
    if #[cfg(feature = "smeg-mcu-arm-")] {
        mod arm;
        pub use arm::{collect_isr_vectors, IsrVectorTableBuilder};
    }
}
