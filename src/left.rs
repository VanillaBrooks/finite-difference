use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct LeftBoundary {
    constant_temperature: T,
}

impl CalculateTemperature for LeftBoundary {
    fn calculate_temperature(&self, _: Information, _: &SolverInfo) -> T {
        self.constant_temperature
    }
}
