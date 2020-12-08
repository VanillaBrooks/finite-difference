mod back;
mod bot;
mod corner;
mod edge;
mod front;
mod internal;
mod left;
mod prelude;
mod right;
mod top;

use back::BackBoundary;
use bot::BottomBoundary;
use front::FrontBoundary;
use left::LeftBoundary;
use right::RightBoundary;
use top::TopBoundary;

use prelude::{CalculateTemperature, SolverInfo, T};

fn main() {
    let temp = 273. + 50.;
    let flux = 10.;
    let h = 20.;
    let t_inf = 400.;
    let energy_generation = 1.;
    let thermal_conductivity = 3.;

    let front = FrontBoundary::builder().constant_temperature(temp).build();
    let back = BackBoundary::builder().constant_temperature(temp).build();
    let left = LeftBoundary::builder().constant_temperature(temp).build();
    let right = RightBoundary::builder().heat_flux(flux).build();
    let top = TopBoundary::builder().h(h).t_inf(t_inf).build();
    let bot = BottomBoundary::builder().constant_temperature(temp).build();
    let walls = WallConditions {
        front,
        back,
        left,
        right,
        top,
        bot,
    };

    let edge_one = edge::EdgeOne::builder()
        .heat_flux(flux)
        .h(h)
        .t_inf(t_inf)
        .build();

    let edges = EdgeConditions { one: edge_one };

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
        divisions: 10,
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

fn solver(s: SolverInfo, params: SolverParams, conditions: BoundaryConditions) {
    let mut previous_temps: ndarray::Array3<f64> =
        ndarray::Array3::zeros((params.divisions, params.divisions, params.divisions));

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
                        // the edge
                        (x_, y_, _) if x_ == div_end && y_ == div_end => {
                            conditions.edges.one.calculate_temperature(information, &s)
                        }
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

        previous_temps = current_temps;
        dbg! {&previous_temps};
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
struct BoundaryConditions {
    walls: WallConditions,
    edges: EdgeConditions,
    internal: internal::InternalConduction,
}

#[derive(typed_builder::TypedBuilder)]
struct WallConditions {
    front: FrontBoundary,
    back: BackBoundary,
    left: LeftBoundary,
    right: RightBoundary,
    top: TopBoundary,
    bot: BottomBoundary,
}

#[derive(typed_builder::TypedBuilder)]
struct EdgeConditions {
    one: edge::EdgeOne,
}
