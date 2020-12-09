use crate::prelude::*;

pub struct LeftSurface<V: BoundaryCondition> {
    pub left_boundary: V,
}

impl<V> CalculateTemperature for LeftSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_front / s.x2())
            + (info.j_back / (2.0 * s.y2()))
            + (info.j_front / (2.0 * s.y2()))
            + (info.k_front / (2.0 * s.z2()))
            + (info.k_back / (2.0 * s.z2()))
            + (self.left_boundary.lhs_constant(&info, s));

        let numerator = m + (s.q_dot / (2.0 * s.k));

        let div = (1. / s.x2())
            + (1. / s.z2())
            + (1. / s.y2())
            + self.left_boundary.rhs_constant(&info, s);

        numerator / div
    }
}
