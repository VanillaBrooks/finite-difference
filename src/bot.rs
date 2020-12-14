use crate::prelude::*;

pub struct BottomSurface<V: BoundaryCondition> {
    pub bot_boundary: V,
}

impl<V> CalculateTemperature for BottomSurface<V>
where
    V: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let area = s.del2();

        let m = (info.i_front + info.i_back + info.k_front + info.k_back) / 2.
            + info.j_front
            + self.bot_boundary.lhs_constant(&info, &s, area);

        let numerator = m + (s.q_dot * s.del2() / (2.0 * s.k));

        let div = 3. + self.bot_boundary.rhs_constant(&info, s, area);

        numerator / div
    }
}
