use crate::prelude::*;
pub struct InfinityNorm;

impl CalculateError for InfinityNorm {
    fn calculate_error(&self, previous_step: &Matrix, current_step: &Matrix) -> T {
        let diff = current_step - previous_step;
        diff.into_iter()
            .map(|x| x.abs())
            .max_by(|left, right| left.partial_cmp(right).unwrap())
            .unwrap()
    }
    fn to_error_type(&self) -> crate::dump::ErrorType {
        crate::dump::ErrorType::InfinityNorm
    }
}

pub struct L1Norm;

impl CalculateError for L1Norm {
    fn calculate_error(&self, previous_step: &Matrix, current_step: &Matrix) -> T {
        let diff = current_step - previous_step;
        diff.into_iter().map(|x| x.abs()).sum::<T>().sqrt()
    }
    fn to_error_type(&self) -> crate::dump::ErrorType {
        crate::dump::ErrorType::L1Norm
    }
}

pub struct L2Norm;

impl CalculateError for L2Norm {
    fn calculate_error(&self, previous_step: &Matrix, current_step: &Matrix) -> T {
        let diff = current_step - previous_step;
        diff.into_iter().map(|x| x.abs().powi(2)).sum::<T>().sqrt()
    }
    fn to_error_type(&self) -> crate::dump::ErrorType {
        crate::dump::ErrorType::L2Norm
    }
}
