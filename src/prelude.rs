pub type T = f64;

pub trait CalculateTemperature {
    fn calculate_temperature(&self, info: Information, solver: &SolverInfo) -> T;
}

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
    pub(crate) del_x: T,
    pub(crate) del_y: T,
    pub(crate) del_z: T,
}
impl SolverInfo {
    pub(crate) fn x2(&self) -> T {
        self.del_x * self.del_x
    }

    pub(crate) fn y2(&self) -> T {
        self.del_y * self.del_y
    }
    pub(crate) fn z2(&self) -> T {
        self.del_z * self.del_z
    }
}
