use core::marker::PhantomData;

use smeg_kernel::errors::{error_tag, KernelError, KernelErrorCode, TaggedError};
use smeg_kernel::syscalls::SyscallResult;

use crate::Dependencies;

use super::SyscallIsrTrampolinePtr;

pub unsafe trait SyscallIsrDispatcher<D: Dependencies> {
    unsafe fn dispatch_syscall(isr_context: &mut D::IsrContext, id: usize, args: usize) -> SyscallResult;
}

pub struct DefaultSyscallIsrDispatcher<D: Dependencies> {
    _dependencies: PhantomData<D>
}

unsafe impl<D: Dependencies> SyscallIsrDispatcher<D> for DefaultSyscallIsrDispatcher<D> {
    unsafe fn dispatch_syscall(isr_context: &mut D::IsrContext, id: usize, args: usize) -> SyscallResult {
        const {
            assert!(
                size_of::<SyscallIsrTrampolinePtr<D::IsrContext>>() == size_of::<usize>(),
                "This code makes the assumption that a function pointer (SyscallIsrTrampolinePtr) can fit into a single machine word");

            assert!(
                size_of::<SyscallIsrTrampolinePtr<D::IsrContext>>() == align_of::<SyscallIsrTrampolinePtr<D::IsrContext>>(),
                r#"This code makes the assumption that SyscallIsrTrampolinePtr size and alignment are identical (ie. a single field) to allow a quick
                runtime check (ie. single comparison), otherwise it would be possible to pass something bad with correct alignment but doesn't point to
                the start of the struct, which may not be a simple power-of-two (as alignment is guaranteed to be).  This scenario would preclude a single
                quick bitwise AND mask test"#);
        }

        let trampoline_vector_table = D::trampoline_vector_table();
        if trampoline_vector_table.is_none_or(|table| table.len() == 0) {
            return Err(KernelError::from(TaggedError::new(KernelErrorCode::UnknownSyscall, error_tag!("no syscalls; possibly no drivers or a linker script error ?"))));
        }

        if id & (align_of::<SyscallIsrTrampolinePtr<D::IsrContext>>() - 1) != 0 {
            return Err(KernelError::from(TaggedError::new(KernelErrorCode::UnknownSyscall, error_tag!("incorrect alignment for Syscall ID argument"))));
        }

        let trampoline_vector_table = trampoline_vector_table.unwrap();
        let first_vector = &raw const *trampoline_vector_table.first().unwrap();
        let last_vector = &raw const *trampoline_vector_table.last().unwrap();
        let trampoline_vector = id as *const SyscallIsrTrampolinePtr<D::IsrContext>;
        if trampoline_vector < first_vector || trampoline_vector > last_vector {
            return Err(KernelError::from(TaggedError::new(KernelErrorCode::UnknownSyscall, error_tag!("Syscall ID is outside the trampoline vector table"))));
        }

        unsafe { (*trampoline_vector)(isr_context, args) }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_kernel::errors::{ErrorTag, KernelErrorCode, ResultToUsizeResultConversion, UsizeResultConversions};

    use smeg_testing_host_utils::integers::any_usize;

    use super::*;

    struct Driver;

    type Dummy = crate::mcu::IsrContext;

    impl Dependencies for Driver {
        type IsrContext = Dummy;
        fn trampoline_vector_table<'a>() -> Option<&'a [SyscallIsrTrampolinePtr<Self::IsrContext>]> {
            static VECTORS: [SyscallIsrTrampolinePtr<Dummy>; 3] = [dummy_trampoline; 3];
            Some(&VECTORS)
        }
    }

    unsafe fn dummy_trampoline(_isr_context: &mut Dummy, _args: usize) -> SyscallResult { Ok(()) }

    #[test]
    fn dispatch_syscall__called_when_trampoline_vector_table_is_none__expect_unknown_syscall_err_is_returned() {
        struct Driver;

        impl Dependencies for Driver {
            type IsrContext = Dummy;
            fn trampoline_vector_table<'a>() -> Option<&'a [SyscallIsrTrampolinePtr<Self::IsrContext>]> { None }
        }

        let id = any_usize() & !(align_of::<SyscallIsrTrampolinePtr<Dummy>>() - 1);
        let mut isr_context = Dummy { };
        let args = Dummy { };
        let result = unsafe {
            DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(&mut isr_context, id, &raw const args as usize)
        };

        expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnknownSyscall);
    }

    #[test]
    fn dispatch_syscall__called_when_trampoline_vector_table_is_empty__expect_unknown_syscall_err_is_returned() {
        struct Driver;

        impl Dependencies for Driver {
            type IsrContext = Dummy;
            fn trampoline_vector_table<'a>() -> Option<&'a [SyscallIsrTrampolinePtr<Self::IsrContext>]> { Some(&[]) }
        }

        let trampolines = Driver::trampoline_vector_table().unwrap();
        let id = &raw const trampolines as usize;
        let mut isr_context = Dummy { };
        let args = Dummy { };
        let result = unsafe {
            DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(&mut isr_context, id, &raw const args as usize)
        };

        expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnknownSyscall);
    }

    #[test]
    fn dispatch_syscall__called_with_unaligned_id__expect_unknown_syscall_err_is_returned() {
        let trampoline = Driver::trampoline_vector_table().unwrap()[1];
        let bad_alignment = align_of::<SyscallIsrTrampolinePtr<Dummy>>() as isize - 1;
        for bad_alignment in -bad_alignment..=bad_alignment {
            if bad_alignment == 0 {
                continue;
            }

            let unaligned_id = (&raw const trampoline as isize + bad_alignment) as usize;
            let mut isr_context = Dummy { };
            let args = Dummy { };
            let result = unsafe {
                DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(&mut isr_context, unaligned_id, &raw const args as usize)
            };

            expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnknownSyscall);
        }
    }

    #[test]
    fn dispatch_syscall__called_with_id_below_address_of_first_trampoline_vector__expect_unknown_syscall_err_is_returned() {
        const PTR_SIZE: isize = size_of::<SyscallIsrTrampolinePtr<Dummy>>() as isize;
        let trampoline_vector_table = Driver::trampoline_vector_table().unwrap();
        let first_vector = trampoline_vector_table.first().unwrap();
        let first_vector = &raw const *first_vector as isize;
        for index_below in [1, 2, 7, 16] {
            let out_of_bounds_id = (first_vector - index_below * PTR_SIZE) as usize;
            let mut isr_context = Dummy { };
            let args = Dummy { };
            let result = unsafe {
                DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(&mut isr_context, out_of_bounds_id, &raw const args as usize)
            };

            expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnknownSyscall);
        }
    }

    #[test]
    fn dispatch_syscall__called_with_id_above_address_of_last_trampoline_vector__expect_unknown_syscall_err_is_returned() {
        const PTR_SIZE: usize = size_of::<SyscallIsrTrampolinePtr<Dummy>>();
        let trampoline_vector_table = Driver::trampoline_vector_table().unwrap();
        let last_vector = trampoline_vector_table.last().unwrap();
        let last_vector = &raw const *last_vector as usize;
        for index_above in [1, 2, 7, 16] {
            let out_of_bounds_id = (last_vector + index_above * PTR_SIZE) as usize;
            let mut isr_context = Dummy { };
            let args = Dummy { };
            let result = unsafe {
                DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(&mut isr_context, out_of_bounds_id, &raw const args as usize)
            };

            expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnknownSyscall);
        }
    }

    #[test]
    fn dispatch_syscall__called__expect_result_from_trampoline_is_returned() {
        struct Stub { _unused: Dummy, pub value: usize, pub tag: ErrorTag }

        impl crate::IsrContext for Stub { }

        impl AsMut<Dummy> for Stub {
            fn as_mut(&mut self) -> &mut Dummy { &mut self._unused }
        }

        impl From<Dummy> for Stub {
            fn from(value: Dummy) -> Self {
                Self { _unused: value, value: 0, tag: error_tag!("Should never be called") }
            }
        }

        struct Driver;
        impl Dependencies for Driver {
            type IsrContext = Stub;
            fn trampoline_vector_table<'a>() -> Option<&'a [SyscallIsrTrampolinePtr<Stub>]> {
                static VECTORS: [SyscallIsrTrampolinePtr<Stub>; 4] = [
                    stub_trampoline::<1>,
                    stub_trampoline::<2>,
                    stub_trampoline::<3>,
                    stub_trampoline::<4>];

                Some(&VECTORS)
            }
        }

        unsafe fn stub_trampoline<const N: usize>(isr_context: &mut Stub, args: usize) -> SyscallResult {
            isr_context.value = isr_context.value.wrapping_mul(N.wrapping_mul(args));
            Err(KernelError::from(TaggedError::new(KernelErrorCode::GeneralSyscallError(byte_xor(isr_context.value)), isr_context.tag)))
        }

        fn byte_xor(x: usize) -> u8 {
            ((x >> 24) ^ (x >> 16) ^ (x >> 8) ^ x) as u8
        }

        let mut isr_context = Stub { _unused: Dummy { }, value: any_usize(), tag: error_tag!() };
        let trampoline_vector_table = Driver::trampoline_vector_table().unwrap();
        let mut id = &raw const trampoline_vector_table[0];
        for index in 0..trampoline_vector_table.len() {
            let args = any_usize();
            let computed_stub = byte_xor(isr_context.value.wrapping_mul((index + 1).wrapping_mul(args)));
            let result = unsafe { DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(&mut isr_context, id as usize, args) }.as_usize_result();
            id = unsafe { id.add(1) };

            let expected_result = Err(TaggedError::new(KernelErrorCode::GeneralSyscallError(computed_stub), isr_context.tag)).as_usize_result();
            expect!(result.as_usize()).to_equal(expected_result.as_usize());
        }
    }
}
