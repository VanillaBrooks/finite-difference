use crate::prelude::*;

pub struct BackSurface<V: BoundaryCondition> {
    pub back_boundary: V,
}

impl<V> CalculateTemperature for BackSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(self.back_boundary);

        let area = s.del2();

        let m = (info.j_front + info.j_back + info.i_front + info.i_back) / 2.
            + info.k_front
            + self.back_boundary.lhs_constant(&info, &s, area);

        let numerator = m + (s.q_dot * s.del2() / (2.0 * s.k));

        let div = 3. + self.back_boundary.rhs_constant(&info, s, area);

        numerator / div
    }
}
