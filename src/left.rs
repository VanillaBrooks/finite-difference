use crate::prelude::*;

pub struct LeftSurface<V: BoundaryCondition> {
    pub left_boundary: V,
}

impl<V> CalculateTemperature for LeftSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(self.left_boundary);

        let area = s.del2();

        let m = (info.j_front + info.j_back + info.k_front + info.k_back) / 2.
            + info.i_front
            + self.left_boundary.lhs_constant(&info, &s, area);

        let numerator = m + (s.q_dot * s.del2() / (2.0 * s.k));

        let div = 3. + self.left_boundary.rhs_constant(&info, s, area);

        numerator / div
    }
}
