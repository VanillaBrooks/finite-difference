use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct BackBoundary {
    constant_temperature: T,
}

impl CalculateTemperature for BackBoundary {
    fn calculate_temperature(&self, _: Information, _: &SolverInfo) -> T {
        self.constant_temperature
    }
}
