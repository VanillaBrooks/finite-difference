use crate::prelude::*;

#[derive(typed_builder::TypedBuilder)]
pub struct EdgeOne {
    heat_flux: T,
    h: T,
    t_inf: T,
}

impl CalculateTemperature for EdgeOne {
    fn calculate_temperature(&self, info: Information, s: &SolverInfo) -> T {
        let m = (info.i_back / (2. * s.x2()))
            + (info.j_back / (2. * s.y2()))
            + (info.k_back / (4. * s.z2()))
            + (info.k_front / (4. * s.z2()))
            // i front
            - (self.heat_flux/(s.k * s.del_x * s.del_y * s.del_z))
            // j front
            + (self.h *self.t_inf/ (2. * s.del_x * s.k));

        let div = (2.0 / s.x2()) + (2.0 / s.y2()) + (2.0 / s.z2());

        let numerator = m + (s.q_dot / s.k);

        numerator / div
    }
}

macro_rules! edge {
    ($name:ident) => {
        pub struct $name {
            constant_temperature: T,
        }

        impl CalculateTemperature for $name {
            fn calculate_temperature(&self, _: Information, _: &SolverInfo) -> T {
                self.constant_temperature
            }
        }
    };
}

// code generation for the other edge temperatures
edge! {EdgeTwo}
edge! {EdgeThree}
edge! {EdgeFour}
edge! {EdgeFive}
edge! {EdgeSix}
edge! {EdgeSeven}
edge! {EdgeEight}
