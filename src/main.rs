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
mod setup;
mod solver;
mod top;

use conditions::*;

use prelude::{SolverInfo, T};

fn main() {
    let flux = 100.;
    let h = 5.;
    let t_inf = 273. + 25.;
    let energy_generation = 0.;
    let thermal_conductivity = 43.;

    let top_boundary = HeatFlux { heat_flux: flux };
    let bot_boundary = Convection { h, t_inf };

    let front_boundary = Convection { h, t_inf };
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
        divisions: 30,
        iterations: 500_000,
    };

    let div: T = params.divisions as f64;

    let solver_info = SolverInfo::builder()
        .k(thermal_conductivity)
        .q_dot(energy_generation)
        .del(params.len / div)
        .build();

    solver::solver(solver_info, params, bcs);
}

struct SolverParams {
    len: T,
    divisions: usize,
    pub(crate) iterations: usize,
}
impl SolverParams {
    fn div_end(&self) -> usize {
        self.divisions - 1
    }
}
