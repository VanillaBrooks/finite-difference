mod back;
mod bot;
mod conditions;
mod corner;
mod dump;
mod edge;
mod error;
mod front;
mod internal;
mod left;
mod prelude;
mod right;
mod setup;
mod solver;
mod top;

use conditions::*;

use prelude::{SolverInfo, T};
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let flux = 50.;
    let h = 5.;
    let t_inf = 273. + 25.;
    let energy_generation = 0.;
    let thermal_conductivity = 43.;

    let top_boundary = HeatFlux { heat_flux: flux };
    let bot_boundary = HeatFlux { heat_flux: flux };

    let front_boundary = HeatFlux { heat_flux: flux };
    let back_boundary = Convection { h, t_inf };

    let left_boundary = Convection { h, t_inf };
    let right_boundary = Convection { h, t_inf };

    let setup = setup::SetupConditions {
        right_boundary,
        left_boundary,
        top_boundary,
        bot_boundary,
        front_boundary,
        back_boundary,
    };

    let bcs = setup.make_boundaries();

    let params = SolverParams {
        len: 0.5,
        divisions: 45,
        error_epsilon: 0.00001,
        steps_before_recording: 10000,
    };

    let div: T = params.divisions as f64;

    let solver_info = SolverInfo::builder()
        .k(thermal_conductivity)
        .q_dot(energy_generation)
        .del(params.len / div)
        .build();

    let error = error::InfinityNorm;

    let mut file = std::fs::File::create("results.json").unwrap();
    let result = solver::solver(solver_info, params, bcs, error);
    dbg! {&result};
    serde_json::to_writer(&mut file, &result).unwrap();
    let diff = Instant::now() - now;
    println!("total runtime: {}", diff.as_secs());
}

struct SolverParams {
    len: T,
    divisions: usize,
    pub(crate) error_epsilon: T,
    pub(crate) steps_before_recording: usize,
}
impl SolverParams {
    fn div_end(&self) -> usize {
        self.divisions - 1
    }
}
