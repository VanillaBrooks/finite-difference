mod back;
mod bot;
mod conditions;
mod corner;
mod edge;
mod front;
mod internal;
mod left;
mod prelude;
mod right;
mod top;

use back::BackSurface;
use bot::BottomSurface;
use front::FrontSurface;
use left::LeftSurface;
use right::RightSurface;
use top::TopSurface;

use conditions::*;

use prelude::{BoundaryCondition, CalculateTemperature, SolverInfo, T};

fn main() {
    let flux = -10.;
    let h = 20.;
    let t_inf = 400.;
    let energy_generation = 1.;
    let thermal_conductivity = 3.;

    let top_boundary = conditions::Convection { h, t_inf };
    let bot_boundary = conditions::HeatFlux { heat_flux: 0. };

    let front_boundary = conditions::HeatFlux { heat_flux: 0. };
    let back_boundary = conditions::HeatFlux { heat_flux: 0. };

    let left_boundary = conditions::HeatFlux { heat_flux: 0. };
    let right_boundary = conditions::HeatFlux { heat_flux: flux };

    let front = FrontSurface { front_boundary };
    let back = BackSurface { back_boundary };
    let left = LeftSurface { left_boundary };
    let right = RightSurface { right_boundary };
    let top = TopSurface { top_boundary };
    let bot = BottomSurface { bot_boundary };

    let walls: WallConditions<HeatFlux, HeatFlux, Convection, HeatFlux, HeatFlux, HeatFlux> =
        WallConditions {
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

    let internal = internal::InternalConduction;

    let bcs = BoundaryConditions {
        edges,
        walls,
        internal,
    };

    let params = SolverParams {
        x_len: 1.,
        y_len: 1.,
        z_len: 1.,
        divisions: 30,
    };

    let div: T = params.divisions as f64;

    let solver_info = SolverInfo::builder()
        .k(thermal_conductivity)
        .q_dot(energy_generation)
        .del_x(params.x_len / div)
        .del_y(params.y_len / div)
        .del_z(params.z_len / div)
        .build();

    solver(solver_info, params, bcs);
}

fn solver<A, B, C, D, E, F>(
    s: SolverInfo,
    params: SolverParams,
    conditions: BoundaryConditions<A, B, C, D, E, F>,
) where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    let mut previous_temps: ndarray::Array3<f64> =
        ndarray::Array3::zeros((params.divisions, params.divisions, params.divisions));

    let mut i = 0;

    loop {
        let mut current_temps: ndarray::Array3<f64> =
            ndarray::Array3::zeros((params.divisions, params.divisions, params.divisions));

        for x in 0..params.divisions {
            // we are on right wall
            if x == params.divisions - 1 {}
            for y in 0..params.divisions {
                for z in 0..params.divisions {
                    //println!("x:{},y:{},z:{}", x, y, z);
                    /*
                     *
                     *
                     */

                    let i_back;
                    let j_back;
                    let k_back;

                    let i_front;
                    let j_front;
                    let k_front;

                    /*
                     * check the backward conditions
                     */
                    if x == 0 {
                        i_back = 0.
                    } else {
                        i_back = previous_temps[[x - 1, y, z]];
                    }

                    if y == 0 {
                        j_back = 0.;
                    } else {
                        j_back = previous_temps[[x, y - 1, z]];
                    }

                    if z == 0 {
                        k_back = 0.;
                    } else {
                        k_back = previous_temps[[x, y, z - 1]];
                    }

                    //println!("finished backs");

                    /*
                     * check the forward conditions
                     */
                    if x == params.divisions - 1 {
                        i_front = 0.
                    } else {
                        i_front = previous_temps[[x + 1, y, z]];
                    }

                    //println!("x forward");

                    if y == params.divisions - 1 {
                        j_front = 0.;
                    } else {
                        j_front = previous_temps[[x, y + 1, z]];
                    }
                    //println!("y forward");

                    if z == params.divisions - 1 {
                        k_front = 0.;
                    } else {
                        //dbg! {x,y,z+1};
                        k_front = previous_temps[[x, y, z + 1]];
                    }

                    //println!("finished forwards");

                    let information = prelude::Information {
                        i_back,
                        i_front,
                        j_back,
                        j_front,
                        k_back,
                        k_front,
                    };

                    let div_end = params.div_end();
                    let temp = match (x, y, z) {
                        /*
                         * Start with corners
                         *
                         * */

                        /*
                         * Do edges
                         *
                         * */
                        (x_, y_, _) if x_ == div_end && y_ == div_end => conditions
                            .edges
                            .right_top
                            .calculate_temperature(information, &s),
                        (x_, y_, _) if x_ == 0 && y_ == div_end => conditions
                            .edges
                            .left_top
                            .calculate_temperature(information, &s),
                        (x_, y_, _) if x_ == 0 && y_ == 0 => conditions
                            .edges
                            .left_bot
                            .calculate_temperature(information, &s),
                        (x_, y_, _) if x_ == div_end && y_ == 0 => conditions
                            .edges
                            .right_bot
                            .calculate_temperature(information, &s),

                        (_, y_, z_) if z_ == div_end && y_ == div_end => conditions
                            .edges
                            .front_top
                            .calculate_temperature(information, &s),
                        (_, y_, z_) if z_ == 0 && y_ == div_end => conditions
                            .edges
                            .back_top
                            .calculate_temperature(information, &s),
                        (_, y_, z_) if z_ == 0 && y_ == 0 => conditions
                            .edges
                            .back_bot
                            .calculate_temperature(information, &s),
                        (_, y_, z_) if z_ == div_end && y_ == 0 => conditions
                            .edges
                            .front_bot
                            .calculate_temperature(information, &s),

                        (x_, _, z_) if x_ == 0 && z_ == 0 => conditions
                            .edges
                            .back_left
                            .calculate_temperature(information, &s),
                        (x_, _, z_) if x_ == div_end && z_ == 0 => conditions
                            .edges
                            .back_right
                            .calculate_temperature(information, &s),
                        (x_, _, z_) if x_ == 0 && z_ == div_end => conditions
                            .edges
                            .front_left
                            .calculate_temperature(information, &s),
                        (x_, _, z_) if x_ == div_end && z_ == div_end => conditions
                            .edges
                            .front_right
                            .calculate_temperature(information, &s),

                        /*
                         * Do Walls
                         *
                         * */
                        (0, _, _) => conditions.walls.left.calculate_temperature(information, &s),
                        (x_, _, _) if x_ == div_end => conditions
                            .walls
                            .right
                            .calculate_temperature(information, &s),
                        (_, 0, _) => conditions.walls.bot.calculate_temperature(information, &s),
                        (_, y_, _) if y_ == div_end => {
                            conditions.walls.top.calculate_temperature(information, &s)
                        }
                        (_, _, 0) => conditions.walls.back.calculate_temperature(information, &s),
                        (_, _, z_) if z_ == div_end => conditions
                            .walls
                            .front
                            .calculate_temperature(information, &s),
                        /*
                         * General internal conduction
                         *
                         * */
                        (_, _, _) => conditions.internal.calculate_temperature(information, &s),
                    };

                    current_temps[[x, y, z]] = temp;

                    /*
                     *
                     *
                     *
                     *
                     */
                } // z
            } // y
        } //x

        if i % 1000 == 0 {
            println! {"i:{}", i}
        }

        previous_temps = current_temps;
        if i == 10_000 {
            dbg! {&previous_temps.slice(ndarray::s!(params.div_end(),..,..))};
            break;
        }
        i += 1
    } // loop
}

struct SolverParams {
    x_len: T,
    y_len: T,
    z_len: T,
    divisions: usize,
}
impl SolverParams {
    fn div_end(&self) -> usize {
        self.divisions - 1
    }
}

#[derive(typed_builder::TypedBuilder)]
struct BoundaryConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    walls: WallConditions<A, B, C, D, E, F>,
    edges: EdgeConditions<A, B, C, D, E, F>,
    internal: internal::InternalConduction,
}

#[derive(typed_builder::TypedBuilder)]
struct WallConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    right: RightSurface<A>,
    left: LeftSurface<B>,
    top: TopSurface<C>,
    bot: BottomSurface<D>,
    front: FrontSurface<E>,
    back: BackSurface<F>,
}

#[derive(typed_builder::TypedBuilder)]
struct EdgeConditions<A, B, C, D, E, F>
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
{
    right_top: edge::RightTop<A, C>,
    right_bot: edge::RightBot<A, D>,
    left_top: edge::LeftTop<B, C>,
    left_bot: edge::LeftBot<B, D>,
    front_top: edge::FrontTop<E, C>,
    front_bot: edge::FrontBot<E, D>,
    back_bot: edge::BackBot<F, D>,
    back_top: edge::BackTop<F, C>,
    back_left: edge::BackLeft<F, B>,
    back_right: edge::BackRight<F, A>,
    front_left: edge::FrontLeft<E, B>,
    front_right: edge::FrontRight<E, A>,
}
