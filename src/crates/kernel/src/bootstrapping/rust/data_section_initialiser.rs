use core::mem::MaybeUninit;

pub unsafe trait DataSectionInitialiser {
    unsafe fn load_data_section(&self, ram_start: &mut MaybeUninit<usize>, ram_past_end: &MaybeUninit<usize>, rom_start: &usize);
}

pub(crate) struct DataSectionInitialiserWithChecks;

unsafe impl DataSectionInitialiser for DataSectionInitialiserWithChecks {
    unsafe fn load_data_section(&self, ram_start: &mut MaybeUninit<usize>, ram_past_end: &MaybeUninit<usize>, rom_start: &usize) {
        unsafe {
// TODO: NEEDS
// despair!(with(KernelErrorCode::LinkerScriptDespair), because("Linker-supplied section pointers for .data are corrupt")
            DataSectionInitialiserWithoutChecks.load_data_section(ram_start, ram_past_end, rom_start)
        }
    }
}

pub(crate) struct DataSectionInitialiserWithoutChecks;

unsafe impl DataSectionInitialiser for DataSectionInitialiserWithoutChecks {
    unsafe fn load_data_section(&self, ram_start: &mut MaybeUninit<usize>, ram_past_end: &MaybeUninit<usize>, rom_start: &usize) {
        unsafe {
/* TODO: FROM THE VERSION WE STUCK ON THE NUCLEO BOARD TO VERIFY:
            let data_size_words = ram_past_end.as_ptr().offset_from(ram_start.as_ptr());
            core::ptr::copy_nonoverlapping(rom_start as *const usize, ram_start.as_mut_ptr(), data_size_words as usize);
*/
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    // TODO: clearly...
}
