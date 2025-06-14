#![doc = crate::docs::side_by_side_md!()]

use crate::bootstrapping::kernel::BoardMcuBootstrapping;

use crate::test_doubles::Dummy;

impl BoardMcuBootstrapping for Dummy {
    type IsrBootstrapper = Dummy;
}
