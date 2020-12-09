use crate::prelude::*;

#[derive(Copy, Clone)]
pub(crate) struct Convection {
    pub(crate) h: T,
    pub(crate) t_inf: T,
}
impl BoundaryCondition for Convection {
    fn lhs_constant(&self, _: &Information, s: &SolverInfo) -> T {
        // TODO: change this del_y to be generic
        self.h * self.t_inf / (s.k * s.del_y)
    }
    fn rhs_constant(&self, _: &Information, s: &SolverInfo) -> T {
        self.h / (s.k * s.del_y)
    }
}

pub trait BoundaryCondition {
    fn lhs_constant(&self, info: &Information, s: &SolverInfo) -> T;
    fn rhs_constant(&self, info: &Information, s: &SolverInfo) -> T;
}

#[derive(Copy, Clone)]
pub(crate) struct HeatFlux {
    pub(crate) heat_flux: T,
}
impl BoundaryCondition for HeatFlux {
    fn lhs_constant(&self, _: &Information, s: &SolverInfo) -> T {
        -1. * self.heat_flux / (s.k * s.del_x * s.del_y * s.del_z)
    }
    fn rhs_constant(&self, _: &Information, _: &SolverInfo) -> T {
        0.
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Temperature {
    temperature: T,
}

//impl BoundaryCondition for Temperature { }
