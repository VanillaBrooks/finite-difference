use crate::prelude::*;
pub struct InternalConduction;

impl CalculateTemperature for InternalConduction {
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m =
            info.i_back + info.i_front + info.j_back + info.j_front + info.k_back + info.k_front;

        let div = 6.;

        let numerator = m + (s.q_dot * s.del2() / s.k);

        numerator / div
    }
}
