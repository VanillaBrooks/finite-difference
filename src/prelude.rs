pub type T = f64;

pub trait CalculateTemperature {
    fn calculate_temperature(&self, info: Information, solver: &SolverInfo) -> T;
}

pub use crate::conditions::BoundaryCondition;

/// Temperature information for the nodes around the current node
///
/// x_front denotes a temperature at x + 1
/// x_back denotes a temperature at x - 1
#[derive(typed_builder::TypedBuilder)]
pub struct Information {
    pub(crate) i_front: T,
    pub(crate) i_back: T,
    pub(crate) j_front: T,
    pub(crate) j_back: T,
    pub(crate) k_front: T,
    pub(crate) k_back: T,
}

#[derive(typed_builder::TypedBuilder, Clone)]
pub struct SolverInfo {
    pub(crate) k: T,
    pub(crate) q_dot: T,
    pub(crate) del: T,
}
impl SolverInfo {
    pub(crate) fn del2(&self) -> T {
        self.del * self.del
    }
}

pub(crate) trait CalculateError {
    fn calculate_error(&self, previous_step: &Matrix, current_step: &Matrix) -> T;
    fn to_error_type(&self) -> crate::dump::ErrorType;
}

pub type Matrix = ndarray::Array3<T>;

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

pub(crate) fn max_temp<ITER, ITEM>(matrix: ITER) -> ITEM
where
    ITER: Iterator<Item = ITEM>,
    ITEM: PartialOrd<ITEM>,
{
    matrix
        .max_by(|left, right| left.partial_cmp(right).unwrap())
        .unwrap()
}
pub(crate) fn min_temp<ITER, ITEM>(matrix: ITER) -> ITEM
where
    ITER: Iterator<Item = ITEM>,
    ITEM: PartialOrd<ITEM>,
{
    matrix
        .min_by(|left, right| left.partial_cmp(right).unwrap())
        .unwrap()
}
