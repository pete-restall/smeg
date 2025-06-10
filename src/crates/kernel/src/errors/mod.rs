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
