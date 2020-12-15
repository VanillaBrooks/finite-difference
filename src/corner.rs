use crate::prelude::*;

pub struct RightTopBack<A, B, C> {
    pub(crate) right_condition: A,
    pub(crate) top_condition: B,
    pub(crate) back_condition: C,
}

impl<A, B, C> CalculateTemperature for RightTopBack<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(
            self.right_condition,
            self.top_condition,
            self.back_condition
        );

        let area = s.del2() / 4.;

        let m = ((info.i_back + info.j_back + info.k_front) / 4.)
            + self.right_condition.lhs_constant(&info, s, area)
            + self.top_condition.lhs_constant(&info, s, area)
            + self.back_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.right_condition.rhs_constant(&info, s, area)
            + self.top_condition.rhs_constant(&info, s, area)
            + self.back_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}

pub struct RightTopFront<A, B, C> {
    pub(crate) right_condition: A,
    pub(crate) top_condition: B,
    pub(crate) front_condition: C,
}

impl<A, B, C> CalculateTemperature for RightTopFront<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(
            self.right_condition,
            self.top_condition,
            self.front_condition
        );

        let area = s.del2() / 4.;

        let m = ((info.i_back + info.j_back + info.k_back) / 4.)
            + self.right_condition.lhs_constant(&info, s, area)
            + self.top_condition.lhs_constant(&info, s, area)
            + self.front_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.right_condition.rhs_constant(&info, s, area)
            + self.top_condition.rhs_constant(&info, s, area)
            + self.front_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}

pub struct RightBottomBack<A, B, C> {
    pub(crate) right_condition: A,
    pub(crate) bot_condition: B,
    pub(crate) back_condition: C,
}

impl<A, B, C> CalculateTemperature for RightBottomBack<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(
            self.right_condition,
            self.bot_condition,
            self.back_condition
        );

        let area = s.del2() / 4.;

        let m = ((info.i_back + info.j_front + info.k_front) / 4.)
            + self.right_condition.lhs_constant(&info, s, area)
            + self.bot_condition.lhs_constant(&info, s, area)
            + self.back_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.right_condition.rhs_constant(&info, s, area)
            + self.bot_condition.rhs_constant(&info, s, area)
            + self.back_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}

pub struct RightBottomFront<A, B, C> {
    pub(crate) right_condition: A,
    pub(crate) bot_condition: B,
    pub(crate) front_condition: C,
}

impl<A, B, C> CalculateTemperature for RightBottomFront<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(
            self.right_condition,
            self.bot_condition,
            self.front_condition
        );

        let area = s.del2() / 4.;

        let m = ((info.i_back + info.j_front + info.k_back) / 4.)
            + self.right_condition.lhs_constant(&info, s, area)
            + self.bot_condition.lhs_constant(&info, s, area)
            + self.front_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.right_condition.rhs_constant(&info, s, area)
            + self.bot_condition.rhs_constant(&info, s, area)
            + self.front_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}
pub struct LeftTopBack<A, B, C> {
    pub(crate) left_condition: A,
    pub(crate) top_condition: B,
    pub(crate) back_condition: C,
}

impl<A, B, C> CalculateTemperature for LeftTopBack<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(self.left_condition, self.top_condition, self.back_condition);
        let area = s.del2() / 4.;

        let m = ((info.i_front + info.j_back + info.k_front) / 4.)
            + self.left_condition.lhs_constant(&info, s, area)
            + self.top_condition.lhs_constant(&info, s, area)
            + self.back_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.left_condition.rhs_constant(&info, s, area)
            + self.top_condition.rhs_constant(&info, s, area)
            + self.back_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}

pub struct LeftTopFront<A, B, C> {
    pub(crate) left_condition: A,
    pub(crate) top_condition: B,
    pub(crate) front_condition: C,
}

impl<A, B, C> CalculateTemperature for LeftTopFront<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(self.left_condition, self.top_condition, self.top_condition);

        let area = s.del2() / 4.;

        let m = ((info.i_front + info.j_back + info.k_back) / 4.)
            + self.left_condition.lhs_constant(&info, s, area)
            + self.top_condition.lhs_constant(&info, s, area)
            + self.front_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.left_condition.rhs_constant(&info, s, area)
            + self.top_condition.rhs_constant(&info, s, area)
            + self.front_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}
pub struct LeftBottomBack<A, B, C> {
    pub(crate) left_condition: A,
    pub(crate) bot_condition: B,
    pub(crate) back_condition: C,
}

impl<A, B, C> CalculateTemperature for LeftBottomBack<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(self.left_condition, self.bot_condition, self.back_condition);

        let area = s.del2() / 4.;

        let m = ((info.i_front + info.j_front + info.k_front) / 4.)
            + self.left_condition.lhs_constant(&info, s, area)
            + self.bot_condition.lhs_constant(&info, s, area)
            + self.back_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.left_condition.rhs_constant(&info, s, area)
            + self.bot_condition.rhs_constant(&info, s, area)
            + self.back_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}
pub struct LeftBottomFront<A, B, C> {
    pub(crate) left_condition: A,
    pub(crate) bot_condition: B,
    pub(crate) front_condition: C,
}

impl<A, B, C> CalculateTemperature for LeftBottomFront<A, B, C>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
{
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        constant_temperature!(
            self.left_condition,
            self.bot_condition,
            self.front_condition
        );

        let area = s.del2() / 4.;

        let m = ((info.i_front + info.j_front + info.k_back) / 4.)
            + self.left_condition.lhs_constant(&info, s, area)
            + self.bot_condition.lhs_constant(&info, s, area)
            + self.front_condition.lhs_constant(&info, s, area);

        let q = s.q_dot * s.del2() / 8.;

        let div = 3. / 4.
            + self.left_condition.rhs_constant(&info, s, area)
            + self.bot_condition.rhs_constant(&info, s, area)
            + self.front_condition.rhs_constant(&info, s, area);

        (q + m) / div
    }
}
