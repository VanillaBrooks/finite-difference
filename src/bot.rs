use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct BottomBoundary {
    constant_temperature: T,
}

impl CalculateTemperature for BottomBoundary {
    fn calculate_temperature(&self, _: Information, _: &SolverInfo) -> T {
        self.constant_temperature
    }
}
