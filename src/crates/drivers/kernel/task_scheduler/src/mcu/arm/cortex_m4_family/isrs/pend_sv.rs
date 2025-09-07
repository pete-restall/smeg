use crate::Dependencies;

pub unsafe fn on_pend_sv_isr<D: Dependencies>(_isr_context: &mut D::IsrContext) {
    // TODO...
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    // TODO...
}
