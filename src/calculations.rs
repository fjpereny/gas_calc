
use aga8::gerg2008::Gerg2008;
use aga8::detail::Detail;
use ratatui::widgets::ListItem;
use ratatui::
    style::{
        Color, 
        Stylize
    };

use crate::units::{
    self, 
    PrintUnit,
    set_temperature,
    get_temperature,
};
use crate::App;

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
    let t1;
    let t2;
    if app.use_gerg2008 {
        t1 = app.gerg_inlet_state.t;
        t2 = app.gerg_outlet_state.t;
    } else {
        t1 = app.aga8_inlet_state.t;
        t2 = app.aga8_outlet_state.t;
    }
    let t1 = units::get_temperature(t1, app.units.temp);
    let t2 = units::get_temperature(t2, app.units.temp);
    t2 - t1
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

pub fn isentropic_eff(app: &mut App, hs: f64) -> f64 {
    if app.use_gerg2008 {
        let pr = app.gerg_outlet_state.p / app.gerg_inlet_state.p;
        if pr >= 1.0 {
            let hd = app.gerg_outlet_state.h - app.gerg_inlet_state.h;
            let hds = hs - app.gerg_inlet_state.h;
            if hd == 0.0 {
                return 0.0
            }
            hds / hd
        } else {
            let hd = app.gerg_inlet_state.h - app.gerg_outlet_state.h;
            let hds = app.gerg_inlet_state.h - hs;
            if hds == 0.0 {
                return 0.0
            }
            hd / hds
        }
    } else {
        let pr = app.aga8_outlet_state.p / app.aga8_inlet_state.p;
        if pr >= 1.0 {
            let hd = app.aga8_outlet_state.h - app.aga8_inlet_state.h;
            if hd == 0.0 {
                return 0.0
            }
            let hds = hs - app.aga8_inlet_state.h;
            hds / hd
        } else {
            let hd = app.aga8_inlet_state.h - app.aga8_outlet_state.h;
            let hds = app.aga8_inlet_state.h - hs;
            if hds == 0.0 {
                return 0.0
            }
            hd / hds
        }
    }
}

pub fn compression_polytropic_exp(app: &mut App) -> f64 {
    let pr = pressure_ratio(app);
    let dr = density_ratio(app);
    pr.ln() / dr.ln()
}

pub fn polytropic_eff(app: &mut App) -> f64 {
    let n = compression_polytropic_exp(app);
    let k = ave_cp_cv(app);
    n / (n-1.0) * (k-1.0) / k
}

pub fn work(app: &mut App) -> f64 {
    let hd = enthalpy_change(app);
    hd * app.flow_val
}

pub fn tip_speed(app: &mut App) -> f64 {
    let pi = std::f64::consts::PI;
    pi * app.wheel_diameter * app.rpm / 60.0
}

pub fn isentropic_temp(app: &mut App) -> f64 {
    let t1;
    let t2s;
    let p1;
    let p2;
    let k;
    if app.use_gerg2008 {
        t1 = app.gerg_inlet_state.t;
        p1 = app.gerg_inlet_state.p;
        p2 = app.gerg_outlet_state.p;
        k = (app.gerg_inlet_state.kappa + app.gerg_outlet_state.kappa) / 2.0;
        let pr = p2 / p1;
        t2s = t1 * pr.powf((k-1.0)/k);
    } else {
        t1 = app.aga8_inlet_state.t;
        p1 = app.aga8_inlet_state.p;
        p2 = app.aga8_outlet_state.p;
        k = (app.aga8_inlet_state.kappa + app.aga8_outlet_state.kappa) / 2.0;
        let pr = p2 / p1;
        t2s = t1 * pr.powf((k-1.0)/k);
    }
    t2s
}

pub fn isentropic_enthalpy(app: &mut App, ts: f64) -> f64 {
    let hs;
    if app.use_gerg2008 {
        let mut gas_state = Gerg2008::new();
        gas_state.set_composition(&app.gas_comp);
        gas_state.p = app.gerg_outlet_state.p;
        gas_state.t = ts;
        gas_state.density(0);
        gas_state.properties();
        hs = gas_state.h;
    } else {
        let mut gas_state = Detail::new();
        gas_state.set_composition(&app.gas_comp);
        gas_state.p = app.aga8_outlet_state.p;
        gas_state.t = ts;
        gas_state.density();
        gas_state.properties();
        hs = gas_state.h;
    }
    hs
}

pub fn isentropic_enthalpy_change(app: &mut App, hs: f64) -> f64 {
    let hds;
    if app.use_gerg2008 {
        let h1 = app.gerg_inlet_state.h;
        hds = hs - h1;
    } else {
        let h1 = app.aga8_inlet_state.h;
        hds = hs - h1;
    }
    hds
}

pub fn run_calculations(app: &mut App) -> [Vec<ListItem<'_>>; 3] {
    let pressure_ratio = pressure_ratio(app);
    let temperature_ratio = temperature_ratio(app);
    let hd = enthalpy_change(app);
    let ts = isentropic_temp(app);
    let hs = isentropic_enthalpy(app, ts);
    let hds = isentropic_enthalpy_change(app, hs);
    let isentropic_efficiency = isentropic_eff(app, hs);

    let efficiency_color;
        if isentropic_efficiency > 1.0 || isentropic_efficiency < 0.0 {
            efficiency_color = Color::Red
        } else {
            efficiency_color = Color::LightCyan
        }

    let left_items = vec![   
        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Press Ratio:", pressure_ratio, "[]",
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Temp Ratio:", temperature_ratio, "[]",
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Temp Change:", temperature_change(app), app.units.temp.print_unit(),
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Enthalpy Change:", hd, app.units.energy.print_unit(),
            )
        )
        .fg(Color::LightCyan)
        .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Entropy Change:", entropy_change(app), app.units.entropy.print_unit(),
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}",  
                "Ave Cp/Cv:", ave_cp_cv(app), "[]",
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),
    ];

    let center_items = vec![   
        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Temperature Ts:", ts, app.units.temp.print_unit(),
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Enthalpy Hs:", hs, app.units.energy.print_unit(),
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Efficiency:", isentropic_efficiency, "[]",
            )
        )
            .fg(efficiency_color)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Enthalpy Change:", hds, app.units.energy.print_unit(),
            )
        )
        .fg(Color::LightCyan)
        .bg(Color::Black),
    ];

    let right_items = vec![   
        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Temperature Ts:", ts, app.units.temp.print_unit(),
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Enthalpy Hs:", hs, app.units.energy.print_unit(),
            )
        )
            .fg(Color::LightCyan)
            .bg(Color::Black),

            ListItem::new(
                format!("{:<18} {:.3} {:>}", 
                "Enthalpy Change:", hds, app.units.energy.print_unit(),
            )
        )
        .fg(Color::LightCyan)
        .bg(Color::Black),

        ListItem::new(
            format!("{:<18} {:.3} {:>}", 
                "Efficiency:", isentropic_efficiency, "[]",
            )
        )
            .fg(efficiency_color)
            .bg(Color::Black),
    ];
    
    [left_items, center_items, right_items]
}