use crate::prelude::*;

pub struct FrontSurface<V: BoundaryCondition> {
    pub front_boundary: V,
}

impl<V> CalculateTemperature for FrontSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / (2.0 * s.x2()))
            + (info.k_back / s.z2())
            + (info.j_back / (2.0 * s.y2()))
            + (info.j_front / (2.0 * s.y2()))
            + (info.i_front / (2.0 * s.x2()))
            + (self.front_boundary.lhs_constant(&info, s));

        let numerator = m + (s.q_dot / (2.0 * s.k));

        let div = (1. / s.x2())
            + (1. / s.z2())
            + (1. / s.y2())
            + self.front_boundary.rhs_constant(&info, s);

        numerator / div
    }
}
