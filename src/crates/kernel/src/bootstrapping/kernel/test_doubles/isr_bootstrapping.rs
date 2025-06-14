#![doc = crate::docs::side_by_side_md!()]

use crate::bootstrapping::kernel::IsrBootstrapping;

use crate::test_doubles::Dummy;

impl IsrBootstrapping for Dummy {
    type IsrContext = Dummy;
}
