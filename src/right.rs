use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct RightBoundary {
    heat_flux: T,
}

impl CalculateTemperature for RightBoundary {
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / s.x2())
            + (info.j_back / (2.0 * s.y2()))
            + (info.k_back / (2.0 * s.z2()))
            - self.heat_flux / (s.k * s.del_x * s.del_y * s.del_z)
            + (info.j_front / (2.0 * s.del_y))
            + (info.k_front / (2.0 * s.z2()));

        let numerator = m + (s.q_dot / (2.0 * s.k));

        let div = (1. / s.x2()) + (1. / s.z2()) + (1. / s.y2());

        numerator / div
    }
}
