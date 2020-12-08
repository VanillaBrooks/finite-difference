use crate::prelude::*;
pub struct InternalConduction;

impl CalculateTemperature for InternalConduction {
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / s.x2())
            + (info.j_back / s.y2())
            + (info.k_back / s.z2())
            + (info.i_front / s.x2())
            + (info.j_front / s.y2())
            + (info.k_front / s.z2());

        let div = (2.0 / s.x2()) + (2.0 / s.y2()) + (2.0 / s.z2());

        let numerator = m + (s.q_dot / s.k);

        numerator / div
    }
}
