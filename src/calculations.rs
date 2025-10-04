

use crate::App;


pub fn pressure_ratio(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.p / app.gerg_inlet_state.p
    } else {
        app.aga8_outlet_state.p / app.aga8_inlet_state.p
    }
}