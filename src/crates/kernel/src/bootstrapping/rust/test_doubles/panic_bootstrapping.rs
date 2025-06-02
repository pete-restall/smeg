#![doc = crate::docs::side_by_side_md!()]

use crate::bootstrapping::rust::PanicBootstrapping;

use crate::test_doubles::Dummy;

impl PanicBootstrapping for Dummy { }
