use crate::prelude::*;

pub struct RightSurface<V: BoundaryCondition> {
    pub right_boundary: V,
}

impl<V> CalculateTemperature for RightSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / s.x2())
            + (info.j_back / (2.0 * s.y2()))
            + (info.k_back / (2.0 * s.z2()))
            + self.right_boundary.lhs_constant(&info, s)
            + (info.j_front / (2.0 * s.del_y))
            + (info.k_front / (2.0 * s.z2()));

        let numerator = m + (s.q_dot / (2.0 * s.k));

        let div = (1. / s.x2()) + (1. / s.z2()) + (1. / s.y2());

        numerator / div
    }
}
