use crate::prelude::*;

pub struct RightSurface<V: BoundaryCondition> {
    pub right_boundary: V,
}

impl<V> CalculateTemperature for RightSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let area = s.del2();

        let m = (info.j_front + info.j_back + info.k_front + info.k_back) / 2.
            + info.i_back
            + self.right_boundary.lhs_constant(&info, &s, area);

        let numerator = m + (s.q_dot * s.del2() / (2.0 * s.k));

        let div = 3. + self.right_boundary.rhs_constant(&info, s, area);

        numerator / div
    }
}
