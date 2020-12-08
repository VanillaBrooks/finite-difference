use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct FrontBoundary {
    constant_temperature: T,
}

impl CalculateTemperature for FrontBoundary {
    fn calculate_temperature(&self, _: Information, _: &SolverInfo) -> T {
        self.constant_temperature
    }
}
