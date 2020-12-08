use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct TopBoundary {
    pub(crate) h: T,
    pub(crate) t_inf: T,
}

impl CalculateTemperature for TopBoundary {
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / (2.0 * s.x2()))
            + (info.j_back / s.y2())
            + (info.k_back / (2.0 * s.z2()))
            + (info.i_front / (2.0 * s.x2()))
            + (self.h * self.t_inf / (s.k * s.del_y))
            + (info.k_front / (2.0 * s.z2()));
        let numerator = m + (s.q_dot / (2.0 * s.k));

        let div = (1. / s.x2()) + (1. / s.z2()) + (1. / s.y2()) + (self.h / (s.k * s.del_y));

        numerator / div
    }
}
