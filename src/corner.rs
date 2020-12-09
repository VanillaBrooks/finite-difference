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
        let m = ((info.i_back + info.j_back + info.k_front) / 4.)
            + self.right_condition.lhs_constant(&info, s)
            + self.top_condition.lhs_constant(&info, s)
            + self.back_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.right_condition.rhs_constant(&info, s)
            + self.top_condition.rhs_constant(&info, s)
            + self.back_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_back + info.j_back + info.k_back) / 4.)
            + self.right_condition.lhs_constant(&info, s)
            + self.top_condition.lhs_constant(&info, s)
            + self.front_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.right_condition.rhs_constant(&info, s)
            + self.top_condition.rhs_constant(&info, s)
            + self.front_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_back + info.j_front + info.k_front) / 4.)
            + self.right_condition.lhs_constant(&info, s)
            + self.bot_condition.lhs_constant(&info, s)
            + self.back_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.right_condition.rhs_constant(&info, s)
            + self.bot_condition.rhs_constant(&info, s)
            + self.back_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_back + info.j_front + info.k_front) / 4.)
            + self.right_condition.lhs_constant(&info, s)
            + self.bot_condition.lhs_constant(&info, s)
            + self.front_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.right_condition.rhs_constant(&info, s)
            + self.bot_condition.rhs_constant(&info, s)
            + self.front_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_front + info.j_back + info.k_front) / 4.)
            + self.left_condition.lhs_constant(&info, s)
            + self.top_condition.lhs_constant(&info, s)
            + self.back_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.left_condition.rhs_constant(&info, s)
            + self.top_condition.rhs_constant(&info, s)
            + self.back_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_front + info.j_back + info.k_back) / 4.)
            + self.left_condition.lhs_constant(&info, s)
            + self.top_condition.lhs_constant(&info, s)
            + self.front_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.left_condition.rhs_constant(&info, s)
            + self.top_condition.rhs_constant(&info, s)
            + self.front_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_front + info.j_front + info.k_front) / 4.)
            + self.left_condition.lhs_constant(&info, s)
            + self.bot_condition.lhs_constant(&info, s)
            + self.back_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.left_condition.rhs_constant(&info, s)
            + self.bot_condition.rhs_constant(&info, s)
            + self.back_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
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
        let m = ((info.i_front + info.j_front + info.k_back) / 4.)
            + self.left_condition.lhs_constant(&info, s)
            + self.bot_condition.lhs_constant(&info, s)
            + self.front_condition.lhs_constant(&info, s);

        let div = (((1. / s.x2()) + (1. / s.y2()) + (1. / s.z2())) / 4.)
            + self.left_condition.rhs_constant(&info, s)
            + self.bot_condition.rhs_constant(&info, s)
            + self.front_condition.rhs_constant(&info, s);

        (m + (s.q_dot / (8. * s.k))) / div
    }
}
