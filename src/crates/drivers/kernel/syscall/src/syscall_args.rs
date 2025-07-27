pub trait SyscallArgs: HasSyscallId { }
impl<T: HasSyscallId> SyscallArgs for T { }

pub trait HasSyscallId {
    fn syscall_id() -> usize;
}

#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_kernel::test_doubles::Dummy;

    use super::HasSyscallId;

    impl HasSyscallId for Dummy {
        fn syscall_id() -> usize { 0 }
    }
}
