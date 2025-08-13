pub mod kernel;
pub mod rust;

// TODO: document - a '*Context' allows bounding injected dependencies on specific calling contexts, such as grabbing a HasMcuCoreId from a Bootstrapping Context might be different from the code allowed to run under an ISR Context
pub trait BootstrapperContext { }

impl<T: super::McuSingleCore> BootstrapperContext for T { }
