
use ratatui::widgets::ListItem;
use ratatui::
    style::{
        Color, 
        Stylize
    };

use crate::units::PrintUnit;
use crate::App;

pub fn state_change_mode(app: &App) -> &'static str {
    let pr = pressure_ratio(app);
    if pr < 1.0 {
        "Expansion"
    } else if pr > 1.0 {
        "Compression"
    } else {
        "Isobaric"
    }
}

pub fn density_ratio(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.d / app.gerg_inlet_state.d
    } else {
        app.aga8_outlet_state.d / app.aga8_inlet_state.d
    }
}

pub fn pressure_ratio(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.p / app.gerg_inlet_state.p
    } else {
        app.aga8_outlet_state.p / app.aga8_inlet_state.p
    }
}

pub fn temperature_ratio(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.t / app.gerg_inlet_state.t
    } else {
        app.aga8_outlet_state.t / app.aga8_inlet_state.t
    }
}

pub fn temperature_change(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.t - app.gerg_inlet_state.t
    } else {
        app.aga8_outlet_state.t - app.aga8_inlet_state.t
    }
}

pub fn enthalpy_change(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.h - app.gerg_inlet_state.h
    } else {
        app.aga8_outlet_state.h - app.aga8_inlet_state.h
    }
}

pub fn entropy_change(app: &App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_outlet_state.s - app.gerg_inlet_state.s
    } else {
        app.aga8_outlet_state.s - app.aga8_inlet_state.s
    }
}

pub fn ave_cp_cv(app: &App) -> f64 {
    if app.use_gerg2008 {
        (app.gerg_outlet_state.kappa + app.gerg_inlet_state.kappa) / 2.0
    } else {
        (app.aga8_outlet_state.kappa + app.aga8_inlet_state.kappa) / 2.0
    }
}

pub fn compression_isentropic_eff(app: &mut App) -> f64 {
    let pr = pressure_ratio(app);
    let k = ave_cp_cv(app);
    let t_in;
    let td;
    if app.use_gerg2008 {
        t_in = app.gerg_inlet_state.t;
        td = app.gerg_outlet_state.t - app.gerg_inlet_state.t;
    } else {
        t_in = app.aga8_inlet_state.t;
        td = app.aga8_outlet_state.t - app.aga8_inlet_state.t;
    }
    (pr.powf((k-1.0)/k) - 1.0) * t_in / td
}

pub fn compression_polytropic_exp(app: &mut App) -> f64 {
    let pr = pressure_ratio(app);
    let dr = density_ratio(app);
    pr.ln() / dr.ln()
}

pub fn compression_polytropic_eff(app: &mut App) -> f64 {
    let n = compression_polytropic_exp(app);
    let k = ave_cp_cv(app);
    n / (n-1.0) * (k-1.0) / k
}

pub fn run_calculations(app: &mut App) -> Vec<ListItem<'_>> {
    let pressure_ratio = pressure_ratio(app);
    let temperature_ratio = temperature_ratio(app);

    let items = vec![
        ListItem::new(
            format!("{}", state_change_mode(app))
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),
        
        ListItem::new(
            format!("{:<18} {:.4} {:15} {} {:.4} {}", 
            "Press Ratio:", pressure_ratio, "[]",
            "Polytropic Exp:", compression_polytropic_exp(app), "[]",
        )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.4} {:15} {} {:.4} {}", 
            "Temp Ratio:", temperature_ratio, "[]",
            "Polytropic Eff:", compression_polytropic_eff(app), "[]",
        )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.4} {}", "Temp Change:", 
                temperature_change(app), 
                app.units.temp.print_unit())
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.4} {}", "Enthalpy Change:", 
                enthalpy_change(app), 
                app.units.energy.print_unit())
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.4} {}", "Entropy Change:", 
                entropy_change(app), 
                app.units.entropy.print_unit())
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.4} {}", "Ave Cp/Cv:", 
                ave_cp_cv(app), 
                "[]")
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.4} {}", "Isentropic Eff:", 
                compression_isentropic_eff(app), 
                "[]")
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),
    ];
    items
}