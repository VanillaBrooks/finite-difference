use crate::prelude::*;

pub struct RightTop<V: BoundaryCondition, K: BoundaryCondition> {
    pub right_boundary: V,
    pub top_boundary: K,
}
impl<V, K> CalculateTemperature for RightTop<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / (2. * s.x2()))
            + (info.k_front / (4. * s.z2()))
            + (info.k_back / (4. * s.z2()))
            + (info.j_back / (2. * s.y2()))
            + self.right_boundary.lhs_constant(&info, s)
            + self.top_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.top_boundary.rhs_constant(&info, s)
            + self.right_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct LeftTop<V: BoundaryCondition, K: BoundaryCondition> {
    pub left_boundary: V,
    pub top_boundary: K,
}
impl<V, K> CalculateTemperature for LeftTop<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_front / (2. * s.x2()))
            + (info.k_front / (4. * s.z2()))
            + (info.k_back / (4. * s.z2()))
            + (info.j_back / (2. * s.y2()))
            + self.left_boundary.lhs_constant(&info, s)
            + self.top_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.top_boundary.rhs_constant(&info, s)
            + self.left_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct LeftBot<V: BoundaryCondition, K: BoundaryCondition> {
    pub left_boundary: V,
    pub bot_boundary: K,
}
impl<V, K> CalculateTemperature for LeftBot<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_front / (2. * s.x2()))
            + (info.k_front / (4. * s.z2()))
            + (info.k_back / (4. * s.z2()))
            + (info.j_front / (2. * s.y2()))
            + self.left_boundary.lhs_constant(&info, s)
            + self.bot_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.bot_boundary.rhs_constant(&info, s)
            + self.left_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct RightBot<V: BoundaryCondition, K: BoundaryCondition> {
    pub right_boundary: V,
    pub bot_boundary: K,
}
impl<V, K> CalculateTemperature for RightBot<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / (2. * s.x2()))
            + (info.k_front / (4. * s.z2()))
            + (info.k_back / (4. * s.z2()))
            + (info.j_front / (2. * s.y2()))
            + self.right_boundary.lhs_constant(&info, s)
            + self.bot_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.bot_boundary.rhs_constant(&info, s)
            + self.right_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct FrontTop<V: BoundaryCondition, K: BoundaryCondition> {
    pub front_boundary: V,
    pub top_boundary: K,
}
impl<V, K> CalculateTemperature for FrontTop<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_back / (2. * s.z2()))
            + (info.i_front / (4. * s.x2()))
            + (info.i_back / (4. * s.x2()))
            + (info.j_back / (2. * s.y2()))
            + self.front_boundary.lhs_constant(&info, s)
            + self.top_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.top_boundary.rhs_constant(&info, s)
            + self.front_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct BackTop<V: BoundaryCondition, K: BoundaryCondition> {
    pub back_boundary: V,
    pub top_boundary: K,
}
impl<V, K> CalculateTemperature for BackTop<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_front / (2. * s.z2()))
            + (info.i_front / (4. * s.x2()))
            + (info.i_back / (4. * s.x2()))
            + (info.j_back / (2. * s.y2()))
            + self.back_boundary.lhs_constant(&info, s)
            + self.top_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.top_boundary.rhs_constant(&info, s)
            + self.back_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct BackBot<V: BoundaryCondition, K: BoundaryCondition> {
    pub back_boundary: V,
    pub bot_boundary: K,
}
impl<V, K> CalculateTemperature for BackBot<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_front / (2. * s.z2()))
            + (info.i_front / (4. * s.x2()))
            + (info.i_back / (4. * s.x2()))
            + (info.j_front / (2. * s.y2()))
            + self.back_boundary.lhs_constant(&info, s)
            + self.bot_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.bot_boundary.rhs_constant(&info, s)
            + self.back_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct FrontBot<V: BoundaryCondition, K: BoundaryCondition> {
    pub front_boundary: V,
    pub bot_boundary: K,
}
impl<V, K> CalculateTemperature for FrontBot<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_back / (2. * s.z2()))
            + (info.i_front / (4. * s.x2()))
            + (info.i_back / (4. * s.x2()))
            + (info.j_front / (2. * s.y2()))
            + self.front_boundary.lhs_constant(&info, s)
            + self.bot_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.bot_boundary.rhs_constant(&info, s)
            + self.front_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct FrontRight<V: BoundaryCondition, K: BoundaryCondition> {
    pub front_boundary: V,
    pub right_boundary: K,
}
impl<V, K> CalculateTemperature for FrontRight<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_back / (2. * s.z2()))
            + (info.j_front / (4. * s.y2()))
            + (info.j_back / (4. * s.y2()))
            + (info.i_back / (2. * s.x2()))
            + self.front_boundary.lhs_constant(&info, s)
            + self.right_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.right_boundary.rhs_constant(&info, s)
            + self.front_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct FrontLeft<V: BoundaryCondition, K: BoundaryCondition> {
    pub front_boundary: V,
    pub left_boundary: K,
}
impl<V, K> CalculateTemperature for FrontLeft<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_back / (2. * s.z2()))
            + (info.j_front / (4. * s.y2()))
            + (info.j_back / (4. * s.y2()))
            + (info.i_front / (2. * s.x2()))
            + self.front_boundary.lhs_constant(&info, s)
            + self.left_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.left_boundary.rhs_constant(&info, s)
            + self.front_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}

pub struct BackLeft<V: BoundaryCondition, K: BoundaryCondition> {
    pub back_boundary: V,
    pub left_boundary: K,
}
impl<V, K> CalculateTemperature for BackLeft<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_front / (2. * s.z2()))
            + (info.j_front / (4. * s.y2()))
            + (info.j_back / (4. * s.y2()))
            + (info.i_front / (2. * s.x2()))
            + self.back_boundary.lhs_constant(&info, s)
            + self.left_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.left_boundary.rhs_constant(&info, s)
            + self.left_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}
pub struct BackRight<V: BoundaryCondition, K: BoundaryCondition> {
    pub back_boundary: V,
    pub right_boundary: K,
}
impl<V, K> CalculateTemperature for BackRight<V, K>
where
    V: BoundaryCondition,
    K: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.k_front / (2. * s.z2()))
            + (info.j_front / (4. * s.y2()))
            + (info.j_back / (4. * s.y2()))
            + (info.i_back / (2. * s.x2()))
            + self.back_boundary.lhs_constant(&info, s)
            + self.right_boundary.lhs_constant(&info, s);

        let div = (1. / (2. * s.x2()))
            + (1. / (2. * s.z2()))
            + (1. / (2. * s.y2()))
            + self.right_boundary.rhs_constant(&info, s)
            + self.back_boundary.rhs_constant(&info, s);

        (m + (s.q_dot / (s.k * 4.))) / div
    }
}
