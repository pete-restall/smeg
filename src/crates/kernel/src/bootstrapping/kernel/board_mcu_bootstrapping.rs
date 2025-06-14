use super::IsrBootstrapping;

pub trait BoardMcuBootstrapping {
    type IsrBootstrapper: IsrBootstrapping;
}
