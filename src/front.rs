use crate::prelude::*;

pub struct FrontSurface<V: BoundaryCondition> {
    pub front_boundary: V,
}

impl<V> CalculateTemperature for FrontSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(self.front_boundary);

        let area = s.del2();

        let m = (info.j_front + info.j_back + info.i_front + info.i_back) / 2.
            + info.k_back
            + self.front_boundary.lhs_constant(&info, &s, area);

        let numerator = m + (s.q_dot * s.del2() / (2.0 * s.k));

        let div = 3. + self.front_boundary.rhs_constant(&info, s, area);

        numerator / div
    }
}
