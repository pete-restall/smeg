mod despair;

mod error_tag;
pub use error_tag::*;

mod kernel_error_code;
pub use kernel_error_code::*;

mod tagged_error;
pub use tagged_error::*;

pub use smeg_kernel_procmacro::error_tag;

pub type KernelError = TaggedError<KernelErrorCode>;
