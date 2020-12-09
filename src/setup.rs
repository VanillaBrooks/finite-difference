use crate::conditions::*;
use crate::corner::*;
use crate::internal;

use super::back::BackSurface;
use super::bot::BottomSurface;
use super::front::FrontSurface;
use super::left::LeftSurface;
use super::right::RightSurface;
use super::top::TopSurface;

use super::edge;

pub(crate) struct SetupConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    pub(crate) right_boundary: A,
    pub(crate) left_boundary: B,
    pub(crate) top_boundary: C,
    pub(crate) bot_boundary: D,
    pub(crate) front_boundary: E,
    pub(crate) back_boundary: F,
}
impl<A, B, C, D, E, F> SetupConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition + Copy,
    B: BoundaryCondition + Copy,
    C: BoundaryCondition + Copy,
    D: BoundaryCondition + Copy,
    E: BoundaryCondition + Copy,
    F: BoundaryCondition + Copy,
{
    pub(crate) fn make_boundaries(self) -> BoundaryConditions<A, B, C, D, E, F> {
        let front_boundary = self.front_boundary;
        let back_boundary = self.back_boundary;
        let right_boundary = self.right_boundary;
        let left_boundary = self.left_boundary;
        let top_boundary = self.top_boundary;
        let bot_boundary = self.bot_boundary;

        let front = FrontSurface { front_boundary };
        let back = BackSurface { back_boundary };
        let left = LeftSurface { left_boundary };
        let right = RightSurface { right_boundary };
        let top = TopSurface { top_boundary };
        let bot = BottomSurface { bot_boundary };

        let walls = WallConditions {
            right,
            left,
            top,
            bot,
            front,
            back,
        };

        /*
         * Set up various edges
         */

        let right_top = edge::RightTop {
            right_boundary,
            top_boundary,
        };
        let right_bot = edge::RightBot {
            right_boundary,
            bot_boundary,
        };
        let left_top = edge::LeftTop {
            left_boundary,
            top_boundary,
        };
        let left_bot = edge::LeftBot {
            bot_boundary,
            left_boundary,
        };

        let front_top = edge::FrontTop {
            front_boundary,
            top_boundary,
        };

        let front_bot = edge::FrontBot {
            front_boundary,
            bot_boundary,
        };

        let back_bot = edge::BackBot {
            back_boundary,
            bot_boundary,
        };
        let back_top = edge::BackTop {
            back_boundary,
            top_boundary,
        };
        let back_left = edge::BackLeft {
            back_boundary,
            left_boundary,
        };
        let back_right = edge::BackRight {
            back_boundary,
            right_boundary,
        };
        let front_left = edge::FrontLeft {
            front_boundary,
            left_boundary,
        };
        let front_right = edge::FrontRight {
            front_boundary,
            right_boundary,
        };

        let edges = EdgeConditions {
            right_top,
            right_bot,
            left_top,
            left_bot,
            front_top,
            front_bot,
            back_bot,
            back_top,
            back_left,
            back_right,
            front_left,
            front_right,
        };

        let corners = make_corners(
            right_boundary,
            left_boundary,
            top_boundary,
            bot_boundary,
            front_boundary,
            back_boundary,
        );
        let internal = internal::InternalConduction;

        let bcs = BoundaryConditions {
            edges,
            walls,
            internal,
            corners,
        };

        bcs
    }
}

fn make_corners<A, B, C, D, E, F>(
    right_condition: A,
    left_condition: B,
    top_condition: C,
    bot_condition: D,
    front_condition: E,
    back_condition: F,
) -> CornerConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition + Copy,
    B: BoundaryCondition + Copy,
    C: BoundaryCondition + Copy,
    D: BoundaryCondition + Copy,
    E: BoundaryCondition + Copy,
    F: BoundaryCondition + Copy,
{
    let right_top_back = RightTopBack {
        right_condition,
        top_condition,
        back_condition,
    };

    let right_top_front = RightTopFront {
        right_condition,
        top_condition,
        front_condition,
    };

    let right_bot_back = RightBottomBack {
        right_condition,
        bot_condition,
        back_condition,
    };
    let right_bot_front = RightBottomFront {
        right_condition,
        bot_condition,
        front_condition,
    };

    let left_top_back = LeftTopBack {
        left_condition,
        top_condition,
        back_condition,
    };

    let left_top_front = LeftTopFront {
        left_condition,
        top_condition,
        front_condition,
    };

    let left_bot_back = LeftBottomBack {
        left_condition,
        bot_condition,
        back_condition,
    };
    let left_bot_front = LeftBottomFront {
        left_condition,
        bot_condition,
        front_condition,
    };

    CornerConditions {
        right_top_back,
        right_top_front,
        right_bot_back,
        right_bot_front,
        left_top_back,
        left_top_front,
        left_bot_back,
        left_bot_front,
    }
}

pub(crate) struct BoundaryConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    pub(crate) walls: WallConditions<A, B, C, D, E, F>,
    pub(crate) edges: EdgeConditions<A, B, C, D, E, F>,
    pub(crate) corners: CornerConditions<A, B, C, D, E, F>,
    pub(crate) internal: internal::InternalConduction,
}

pub(crate) struct WallConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    pub(crate) right: RightSurface<A>,
    pub(crate) left: LeftSurface<B>,
    pub(crate) top: TopSurface<C>,
    pub(crate) bot: BottomSurface<D>,
    pub(crate) front: FrontSurface<E>,
    pub(crate) back: BackSurface<F>,
}

pub(crate) struct EdgeConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    pub(crate) right_top: edge::RightTop<A, C>,
    pub(crate) right_bot: edge::RightBot<A, D>,
    pub(crate) left_top: edge::LeftTop<B, C>,
    pub(crate) left_bot: edge::LeftBot<B, D>,
    pub(crate) front_top: edge::FrontTop<E, C>,
    pub(crate) front_bot: edge::FrontBot<E, D>,
    pub(crate) back_bot: edge::BackBot<F, D>,
    pub(crate) back_top: edge::BackTop<F, C>,
    pub(crate) back_left: edge::BackLeft<F, B>,
    pub(crate) back_right: edge::BackRight<F, A>,
    pub(crate) front_left: edge::FrontLeft<E, B>,
    pub(crate) front_right: edge::FrontRight<E, A>,
}

pub(crate) struct CornerConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    pub(crate) right_top_back: RightTopBack<A, C, F>,
    pub(crate) right_top_front: RightTopFront<A, C, E>,
    pub(crate) right_bot_back: RightBottomBack<A, D, F>,
    pub(crate) right_bot_front: RightBottomFront<A, D, E>,
    pub(crate) left_top_back: LeftTopBack<B, C, F>,
    pub(crate) left_top_front: LeftTopFront<B, C, E>,
    pub(crate) left_bot_back: LeftBottomBack<B, D, F>,
    pub(crate) left_bot_front: LeftBottomFront<B, D, E>,
}
