use crate::dump::{ErrorData, SimulationResult, StepData};
use crate::prelude;
use crate::prelude::*;
use crate::setup::BoundaryConditions;
use crate::SolverParams;

pub(crate) fn solver<A, B, C, D, E, F, ErrCalc>(
    s: SolverInfo,
    params: SolverParams,
    conditions: BoundaryConditions<A, B, C, D, E, F>,
    error_type: ErrCalc,
) -> SimulationResult
where
    A: BoundaryCondition,
    B: BoundaryCondition,
    C: BoundaryCondition,
    D: BoundaryCondition,
    E: BoundaryCondition,
    F: BoundaryCondition,
    ErrCalc: CalculateError,
{
    let step_estimation = 10_000;

    let mut error_decay = ErrorData {
        error_type: error_type.to_error_type(),
        data: Vec::with_capacity(step_estimation),
    };
    let mut step_data: Vec<StepData> = Vec::with_capacity(step_estimation);

    let mut previous_temps: ndarray::Array3<f64> =
        ndarray::Array3::ones((params.divisions, params.divisions, params.divisions)) * 273.;

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
                    if x == params.div_end() {
                        i_front = 0.
                    } else {
                        i_front = previous_temps[[x + 1, y, z]];
                    }

                    //println!("x forward");

                    if y == params.div_end() {
                        j_front = 0.;
                    } else {
                        j_front = previous_temps[[x, y + 1, z]];
                    }
                    //println!("y forward");

                    if z == params.div_end() {
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
                        (x_, y_, z_) if x_ == 0 && y_ == 0 && z_ == 0 => conditions
                            .corners
                            .left_bot_back
                            .calculate_temperature(information, &s),
                        (x_, y_, z_) if x_ == 0 && y_ == 0 && z_ == div_end => conditions
                            .corners
                            .left_bot_front
                            .calculate_temperature(information, &s),
                        (x_, y_, z_) if x_ == 0 && y_ == div_end && z_ == 0 => conditions
                            .corners
                            .left_top_back
                            .calculate_temperature(information, &s),
                        (x_, y_, z_) if x_ == 0 && y_ == div_end && z_ == div_end => conditions
                            .corners
                            .left_top_front
                            .calculate_temperature(information, &s),

                        (x_, y_, z_) if x_ == div_end && y_ == 0 && z_ == 0 => conditions
                            .corners
                            .right_bot_back
                            .calculate_temperature(information, &s),
                        (x_, y_, z_) if x_ == div_end && y_ == 0 && z_ == div_end => conditions
                            .corners
                            .right_bot_front
                            .calculate_temperature(information, &s),
                        (x_, y_, z_) if x_ == div_end && y_ == div_end && z_ == 0 => conditions
                            .corners
                            .right_top_back
                            .calculate_temperature(information, &s),
                        (x_, y_, z_) if x_ == div_end && y_ == div_end && z_ == div_end => {
                            conditions
                                .corners
                                .right_top_front
                                .calculate_temperature(information, &s)
                        }

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

        if i % 1_000 == 0 {
            println! {"i:{}", i}
        }

        // check if we need to record this data for plotting
        if i % params.steps_before_recording == 0 {
            let raw_data = current_temps.clone().into_raw_vec();

            let new_data = StepData {
                step: i,
                data: raw_data,
            };

            step_data.push(new_data)
        }

        let curr_error = error_type.calculate_error(&previous_temps, &current_temps);

        error_decay.add_error(curr_error);

        if curr_error < params.error_epsilon {
            // we are below the threshold for error right now, we can quit here

            let result = SimulationResult {
                step_data,
                error_decay,
                size: params.divisions,
            };

            return result;
        }

        previous_temps = current_temps;

        i += 1
    } // loop
}
