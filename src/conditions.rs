use crate::prelude::*;

#[derive(Copy, Clone, serde::Serialize)]
pub(crate) struct Convection {
    pub(crate) h: T,
    pub(crate) t_inf: T,
}
impl BoundaryCondition for Convection {
    fn lhs_constant(&self, _: &Information, s: &SolverInfo, area: T) -> T {
        area * self.h * self.t_inf / (s.k * s.del)
    }
    fn rhs_constant(&self, _: &Information, s: &SolverInfo, area: T) -> T {
        area * self.h / (s.k * s.del)
    }
}

pub trait BoundaryCondition {
    fn lhs_constant(&self, info: &Information, s: &SolverInfo, area: T) -> T;
    fn rhs_constant(&self, info: &Information, s: &SolverInfo, area: T) -> T;
    fn constant_temperature(&self) -> Option<T> {
        None
    }
}

#[derive(Copy, Clone, serde::Serialize)]
pub(crate) struct HeatFlux {
    pub(crate) heat_flux: T,
}
impl BoundaryCondition for HeatFlux {
    fn lhs_constant(&self, _: &Information, s: &SolverInfo, area: T) -> T {
        area * self.heat_flux / (s.k * s.del)
    }
    fn rhs_constant(&self, _: &Information, _: &SolverInfo, _area: T) -> T {
        0.
    }
}

#[derive(Copy, Clone, serde::Serialize)]
pub(crate) struct Temperature {
    #[allow(dead_code)]
    pub(crate) temperature: T,
}

impl BoundaryCondition for Temperature {
    fn lhs_constant(&self, _: &Information, _: &SolverInfo, _: T) -> T {
        unimplemented!()
    }
    fn rhs_constant(&self, _: &Information, _: &SolverInfo, _area: T) -> T {
        unimplemented!()
    }
    fn constant_temperature(&self) -> Option<T> {
        Some(self.temperature)
    }
}
