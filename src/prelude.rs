pub type T = f64;

pub trait CalculateTemperature {
    fn calculate_temperature(&self, info: Information, solver: &SolverInfo) -> T;
}

pub use crate::conditions::BoundaryCondition;

/// Temperature information for the nodes around the current node
///
/// x_front denotes a temperature at x + 1
/// x_back denotes a temperature at x - 1
#[derive(typed_builder::TypedBuilder)]
pub struct Information {
    pub(crate) i_front: T,
    pub(crate) i_back: T,
    pub(crate) j_front: T,
    pub(crate) j_back: T,
    pub(crate) k_front: T,
    pub(crate) k_back: T,
}

#[derive(typed_builder::TypedBuilder, Clone)]
pub struct SolverInfo {
    pub(crate) k: T,
    pub(crate) q_dot: T,
    pub(crate) del: T,
}
impl SolverInfo {
    pub(crate) fn del2(&self) -> T {
        self.del * self.del
    }
}
