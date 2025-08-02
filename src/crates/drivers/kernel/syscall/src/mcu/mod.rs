cfg_if::cfg_if! {
    if #[cfg(any(test, feature = "test_doubles"))] {
        pub mod test_doubles;
    } else if #[cfg(feature = "smeg-mcu-arm-")] {
        mod arm;
        pub use arm::*;
    }
}
