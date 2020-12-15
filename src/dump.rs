use crate::prelude::*;
use std::fmt;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub(crate) struct SimulationResult {
    pub(crate) step_data: Vec<StepData>,
    pub(crate) error_decay: ErrorData,
    // the number of sides in each dimension
    pub(crate) size: usize,
}
impl fmt::Debug for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last = &self.step_data[&self.step_data.len() - 1];
        let min = min_temp(last.data.iter());
        let max = max_temp(last.data.iter());

        f.debug_struct("SimulationResult")
            .field("step_data (latest)", last)
            //.field("error_decay", &self.error_decay)
            .field("size", &self.size)
            .field("min", &min)
            .field("max", &max)
            .finish()
    }
}

#[derive(Clone, Serialize, Debug)]
pub(crate) struct StepData {
    pub(crate) step: usize,
    pub(crate) data: Vec<T>,
}

#[derive(Clone, Serialize, Debug)]
pub(crate) struct ErrorData {
    pub(crate) error_type: ErrorType,
    pub(crate) data: Vec<T>,
}
impl ErrorData {
    pub(crate) fn add_error(&mut self, new_point: T) {
        self.data.push(new_point)
    }
}

#[derive(Clone, Serialize, Debug)]
pub(crate) enum ErrorType {
    InfinityNorm,
    L1Norm,
    L2Norm,
}
