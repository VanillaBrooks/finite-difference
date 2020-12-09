use crate::prelude::*;

pub struct TopSurface<V: BoundaryCondition> {
    pub top_boundary: V,
}

impl<V> CalculateTemperature for TopSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.j_back / s.y2())
            + (info.i_back / (2.0 * s.x2()))
            + (info.i_front / (2.0 * s.x2()))
            + (info.k_back / (2.0 * s.z2()))
            + (info.k_front / (2.0 * s.z2()))
            + (self.top_boundary.lhs_constant(&info, s));
        let numerator = m + (s.q_dot / (2.0 * s.k));

        let div = (1. / s.x2())
            + (1. / s.z2())
            + (1. / s.y2())
            + self.top_boundary.rhs_constant(&info, s);

        numerator / div
    }
}
