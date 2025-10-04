
use aga8::composition::Composition;
use crate::{
    App,
    recalculate,
};

pub enum Gas {
    Air,
    Argon,
    CO,
    CO2,
    Helium,
    Hydrogen,
    Nitrogen,
    Oxygen,
}

pub fn get_gas_comp(gas_comp: Gas) -> Composition{

    match gas_comp {
        Gas::Air => Composition {
            nitrogen: 0.78,
            oxygen: 0.21,
            argon: 0.01,
            ..Default::default()
        },
        Gas::Argon => Composition {
            argon: 1.0,
            ..Default::default()
        },
        Gas::CO => Composition {
            carbon_monoxide: 1.0,
            ..Default::default()
        },
        Gas::CO2 => Composition {
            carbon_dioxide: 1.0,
            ..Default::default()
        },
        Gas::Helium => Composition {
            helium: 1.0,
            ..Default::default()
        },
        Gas::Hydrogen => Composition {
            hydrogen: 1.0,
            ..Default::default()
        },
        Gas::Nitrogen => Composition {
            nitrogen: 1.0,
            ..Default::default()
        },
        Gas::Oxygen => Composition {
            oxygen: 1.0,
            ..Default::default()
        },
    }

}

pub fn set_gas(app: &mut App, composition: Composition) {
    app.aga8_cur_state.set_composition(&composition);
    app.gerg_cur_state.set_composition(&composition);
    app.aga8_inlet_state.set_composition(&composition);
    app.gerg_inlet_state.set_composition(&composition);
    app.aga8_outlet_state.set_composition(&composition);
    app.gerg_outlet_state.set_composition(&composition);
    recalculate(app);
}