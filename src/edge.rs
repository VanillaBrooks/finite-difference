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
        let area = s.del2() / 2.;

        let m = ((info.i_back + info.j_back) / 2.)
            + ((info.k_back + info.k_front) / 4.)
            + self.right_boundary.lhs_constant(&info, s, area)
            + self.top_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.right_boundary.rhs_constant(&info, s, area)
            + self.top_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.i_front + info.j_back) / 2.)
            + ((info.k_back + info.k_front) / 4.)
            + self.left_boundary.lhs_constant(&info, s, area)
            + self.top_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.left_boundary.rhs_constant(&info, s, area)
            + self.top_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.i_front + info.j_front) / 2.)
            + ((info.k_back + info.k_front) / 4.)
            + self.left_boundary.lhs_constant(&info, s, area)
            + self.bot_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.left_boundary.rhs_constant(&info, s, area)
            + self.bot_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.i_back + info.j_front) / 2.)
            + ((info.k_back + info.k_front) / 4.)
            + self.right_boundary.lhs_constant(&info, s, area)
            + self.bot_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.right_boundary.rhs_constant(&info, s, area)
            + self.bot_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_back + info.j_back) / 2.)
            + ((info.i_front + info.i_back) / 4.)
            + self.front_boundary.lhs_constant(&info, s, area)
            + self.top_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.top_boundary.rhs_constant(&info, s, area)
            + self.front_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_front + info.j_back) / 2.)
            + ((info.i_front + info.i_back) / 4.)
            + self.back_boundary.lhs_constant(&info, s, area)
            + self.top_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.top_boundary.rhs_constant(&info, s, area)
            + self.back_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_front + info.j_front) / 2.)
            + ((info.i_front + info.i_back) / 4.)
            + self.back_boundary.lhs_constant(&info, s, area)
            + self.bot_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.bot_boundary.rhs_constant(&info, s, area)
            + self.back_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_back + info.j_front) / 2.)
            + ((info.i_front + info.i_back) / 4.)
            + self.front_boundary.lhs_constant(&info, s, area)
            + self.bot_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.bot_boundary.rhs_constant(&info, s, area)
            + self.front_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
    }
}

//
//
// j edge stuff
//
//

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
        let area = s.del2() / 2.;

        let m = ((info.k_back + info.i_back) / 2.)
            + ((info.j_front + info.j_back) / 4.)
            + self.front_boundary.lhs_constant(&info, s, area)
            + self.right_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.right_boundary.rhs_constant(&info, s, area)
            + self.front_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_back + info.i_front) / 2.)
            + ((info.j_front + info.j_back) / 4.)
            + self.front_boundary.lhs_constant(&info, s, area)
            + self.left_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.left_boundary.rhs_constant(&info, s, area)
            + self.front_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_front + info.i_front) / 2.)
            + ((info.j_front + info.j_back) / 4.)
            + self.back_boundary.lhs_constant(&info, s, area)
            + self.left_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.left_boundary.rhs_constant(&info, s, area)
            + self.back_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
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
        let area = s.del2() / 2.;

        let m = ((info.k_front + info.i_back) / 2.)
            + ((info.j_front + info.j_back) / 4.)
            + self.back_boundary.lhs_constant(&info, s, area)
            + self.right_boundary.lhs_constant(&info, s, area);

        let div = 3. / 2.
            + self.right_boundary.rhs_constant(&info, s, area)
            + self.back_boundary.rhs_constant(&info, s, area);

        (m + (s.q_dot * s.del2() / (s.k * 4.))) / div
    }
}
