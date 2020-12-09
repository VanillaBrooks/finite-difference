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
use corner::*;

use prelude::{BoundaryCondition, CalculateTemperature, SolverInfo, T};

fn main() {
    let flux = -10.;
    let h = 20.;
    let t_inf = 400.;
    let energy_generation = 1.;
    let thermal_conductivity = 3.;

    let top_boundary = Convection { h, t_inf };
    let bot_boundary = HeatFlux { heat_flux: 0. };

    let front_boundary = HeatFlux { heat_flux: 0. };
    let back_boundary = HeatFlux { heat_flux: 0. };

    let left_boundary = HeatFlux { heat_flux: 0. };
    let right_boundary = HeatFlux { heat_flux: flux };

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

    solver::solver(solver_info, params, bcs);
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
