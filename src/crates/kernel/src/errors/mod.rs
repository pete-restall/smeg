mod despair;

mod error_tag;

pub use error_tag::prelude::*;

mod kernel_error_code;
pub use kernel_error_code::prelude::*;

mod tagged_error;
pub use tagged_error::*;

pub use smeg_kernel_procmacro::error_tag;

mod results;
pub use results::prelude::*;

#[allow(unused)]
#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    pub use super::error_tag::test_doubles::*;
    pub use super::kernel_error_code::test_doubles::*;
    pub use super::results::test_doubles::*;
}
