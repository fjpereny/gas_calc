mod units;
mod gas;

use std::fmt::format;

use aga8::detail::Detail;
use aga8::gerg2008::Gerg2008;
use aga8::composition::Composition;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect}, 
    style::{self, Color, Style, Stylize}, 
    symbols,
    widgets::{self, Block, Borders, Clear, List, ListItem, Paragraph,}, 
    Frame
};
use ratatui_textarea::TextArea;

use crate::gas::get_gas_comp;
use crate::units::{Units, PrintUnit};

pub struct App {
    pub pressure_modal_visible: bool,
    pub aga8_cur_state: Detail,
    pub gerg_cur_state: Gerg2008,
    pub aga8_inlet_state: Detail,
    pub gerg_inlet_state: Gerg2008,
    pub aga8_outlet_state: Detail,
    pub gerg_outlet_state: Gerg2008,
    pub gas_comp: Composition,
    pub use_gerg2008: bool,
    pub units: Units,
    pub show_inlet_state: bool,
    pub show_outlet_state: bool,
}

impl Default for App {
    fn default() -> Self {
        App { 
            pressure_modal_visible: false,
            aga8_cur_state: Detail::new(),
            gerg_cur_state: Gerg2008::new(), 
            aga8_inlet_state: Detail::new(),
            gerg_inlet_state: Gerg2008::new(),
            aga8_outlet_state: Detail::new(),
            gerg_outlet_state: Gerg2008::new(),
            gas_comp: Composition::default(),
            use_gerg2008: true,
            units: Units::default(),
            show_inlet_state: false,
            show_outlet_state: false,
        }
    }
}


fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    
    let mut app = App::default();
    app.gas_comp = get_gas_comp(gas::Gas::Air);
    app.aga8_cur_state.set_composition(&app.gas_comp);
    app.gerg_cur_state.set_composition(&app.gas_comp);
    app.aga8_cur_state.p = 100.0;
    app.gerg_cur_state.p = 100.0;
    app.aga8_cur_state.t = 273.15;
    app.gerg_cur_state.t = 273.15;
    app.aga8_cur_state.density();
    app.gerg_cur_state.density(0);
    app.aga8_cur_state.properties();
    app.gerg_cur_state.properties();
    
    let result = run(&mut terminal, &mut app);
    ratatui::restore();
    result
}


fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;
        if handle_events(app)? {
            break Ok(());
        }
    }
}

fn hotkey_menu() -> Paragraph<'static> {
    Paragraph::new(
        format!("Esc-Settings\tP-Pressure\tT-Temperature\tI-Set Inlet\tO- Set Outlet\t C-Clear")
    )
}

fn draw(frame: &mut Frame, app: &mut App) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Length(16), Fill(1), Length(3)]);
    let [title_area, main_area, calc_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 3]);
    let [left_area, center_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Thermodynamic Gas Calculator").style(Color::LightCyan), title_area);
    
    let hotkey_par = hotkey_menu()
        .block(
            Block::bordered()
            .title("Hotkeys")
            .style(Color::LightCyan)
        );
    frame.render_widget(hotkey_par, status_area);
    
    let items = get_gas_properties(app);
    let items_list = List::new(items)
    .block(Block::bordered()
    .title("Current State"));
    frame.render_widget(items_list, left_area);

    if app.show_inlet_state {
        let items = get_gas_properties(app);
        let items_list = List::new(items)
        .block(Block::bordered()
        .title("Inlet State")
        .style(Color::Green)
    );
    frame.render_widget(items_list, center_area);
} else {
    frame.render_widget(
        Block::bordered().title("Inlet State (not defined)")
        .style(Color::Red), 
        center_area);
    }
    
    if app.show_outlet_state {
        let items = get_gas_properties(app);
        let items_list = List::new(items)
            .block(Block::bordered()
            .title("Outlet State")
            .style(Color::Green)
        );
        frame.render_widget(items_list, right_area);
    } else {
        frame.render_widget(
            Block::bordered().title("Outlet State (not defined)")
            .style(Color::Red), 
            right_area);
    }
        
    
    frame.render_widget(Block::bordered().title("Calculations (set inlet and outlet conditions to calculate)"), calc_area);

    if app.pressure_modal_visible { // Replace with your modal visibility condition
        pressure_modal(app, frame, main_area);
    }
}



fn handle_events(app: &mut App) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('p') => app.pressure_modal_visible = !app.pressure_modal_visible,
            KeyCode::Char('i') => set_inlet_conditions(app),
            KeyCode::Char('o') => set_outlet_conditions(app),
            KeyCode::Char('m') => app.use_gerg2008 = ! app.use_gerg2008,
            KeyCode::Char('c') => {
                app.show_inlet_state = false;
                app.show_outlet_state = false;
            },
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}


/// Helper function to create a centered rect using a certain percentage of the available rect `r`.
fn popup_area(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}


fn pressure_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title("Settings")
    .borders(Borders::ALL)
    .style(Style::new().bg(Color::LightBlue));

    let modal_content = Paragraph::new(format!("Enter Pressure {}", app.units.pressure.print_unit()))
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

fn get_pressure(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_pressure(app.gerg_cur_state.p, app.units.pressure)
    } else {
        units::get_pressure(app.aga8_cur_state.p, app.units.pressure)
    }
}

fn set_cur_pressure(pressure: f64, app: &mut App) {
    let p_val = units::set_pressure(pressure, app.units.pressure);
    app.aga8_cur_state.p = p_val;
    app.gerg_cur_state.p = p_val;
}

fn set_pressure(app: &mut App, inlet: bool) {
    if inlet {
        app.aga8_inlet_state.p = units::set_pressure(app.aga8_cur_state.p, app.units.pressure);
        app.gerg_inlet_state.p = units::set_pressure(app.gerg_cur_state.p, app.units.pressure);
    } else {
        app.aga8_outlet_state.p = units::set_pressure(app.aga8_cur_state.p, app.units.pressure);
        app.gerg_outlet_state.p = units::set_pressure(app.gerg_cur_state.p, app.units.pressure);
    }
}

fn get_temperature(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_temperature(app.gerg_cur_state.t, app.units.temp)
    } else {
        units::get_temperature(app.aga8_cur_state.t, app.units.temp)
    }
}

fn set_temperature(app: &mut App, inlet: bool) {
    if inlet {
        app.aga8_inlet_state.t = units::set_temperature(app.aga8_cur_state.t, app.units.temp);
        app.gerg_inlet_state.t = units::set_temperature(app.gerg_cur_state.t, app.units.temp);
    } else {
        app.aga8_outlet_state.t = units::set_temperature(app.aga8_cur_state.t, app.units.temp);
        app.gerg_outlet_state.t = units::set_temperature(app.gerg_cur_state.t, app.units.temp);
    }
}

fn get_density(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_cur_state.d
    } else {
        app.aga8_cur_state.d
    }
}

fn get_molar_mass(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        app.gerg_cur_state.mm
    } else {
        app.aga8_cur_state.mm
    }
}

fn get_internal_energy(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_energy(app.gerg_cur_state.u, app.units.energy, app.gerg_cur_state.mm)
    } else {
        units::get_energy(app.aga8_cur_state.u, app.units.energy, app.aga8_cur_state.mm)
    }
}

fn get_enthalpy(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_energy(app.gerg_cur_state.h, app.units.energy, app.gerg_cur_state.mm)
    } else {
        units::get_energy(app.aga8_cur_state.h, app.units.energy, app.aga8_cur_state.mm)
    }
}

fn get_entropy(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_entropy(app.gerg_cur_state.s, app.units.entropy, app.gerg_cur_state.mm)
    } else {
        units::get_entropy(app.aga8_cur_state.s, app.units.entropy, app.aga8_cur_state.mm)
    }
}

fn get_cp(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_entropy(app.gerg_cur_state.cp, app.units.entropy, app.gerg_cur_state.mm)
    } else {
        units::get_entropy(app.aga8_cur_state.cp, app.units.entropy, app.aga8_cur_state.mm)
    }
}

fn get_cv(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_entropy(app.gerg_cur_state.cv, app.units.entropy, app.gerg_cur_state.mm)
    } else {
        units::get_entropy(app.aga8_cur_state.cv, app.units.entropy, app.aga8_cur_state.mm)
    }
}

fn get_speed(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_speed(app.gerg_cur_state.w, app.units.speed)
    } else {
        units::get_speed(app.aga8_cur_state.w, app.units.speed)
    }
}

fn get_gibbs_energy(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_gibbs_energy(app.gerg_cur_state.g, app.units.pressure, app.units.temp)
    } else {
        units::get_gibbs_energy(app.aga8_cur_state.g, app.units.pressure, app.units.temp)
    }
}

fn get_jt_coeff(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_jt_coeff(app.gerg_cur_state.jt, app.units.jt_coeff)
    } else {
        units::get_jt_coeff(app.aga8_cur_state.jt, app.units.jt_coeff)
    }
}

fn set_inlet_conditions(app: &mut App) {
    app.aga8_inlet_state.set_composition(&copy_composition(&app.gas_comp));
    app.gerg_inlet_state.set_composition(&copy_composition(&app.gas_comp));
    set_pressure(app, true);
    set_temperature(app, true);
    app.show_inlet_state = true;
}

fn set_outlet_conditions(app: &mut App) {
    app.aga8_outlet_state.set_composition(&copy_composition(&app.gas_comp));
    app.gerg_outlet_state.set_composition(&copy_composition(&app.gas_comp));
    set_pressure(app, false);
    set_temperature(app, false);
    app.show_outlet_state = true;
}

fn copy_composition(composition: &Composition) -> Composition {
    let mut comp = Composition::default();
    comp.argon = composition.argon;
    comp.carbon_dioxide = composition.carbon_dioxide;
    comp.carbon_monoxide = composition.carbon_monoxide;
    comp.decane = composition.decane;
    comp.ethane = composition.ethane;
    comp.helium = composition.helium;
    comp.heptane = composition.heptane;
    comp.hexane = composition.hexane;
    comp.hydrogen = composition.hydrogen;
    comp.hydrogen_sulfide = composition.hydrogen_sulfide;
    comp.isobutane = composition.isobutane;
    comp.isopentane = composition.isopentane;
    comp.methane = composition.methane;
    comp.n_butane = composition.n_butane;
    comp.n_pentane = composition.n_pentane;
    comp.nitrogen = composition.nitrogen;
    comp.nonane = composition.nonane;
    comp.octane = composition.octane;
    comp.propane = composition.propane;
    comp.water = composition.water;
    comp
}

fn get_gas_properties(app: &App) -> Vec<ListItem> {
        let mut p;
        let p_str = app.units.pressure.print_unit();
        let mut t;
        let t_str = app.units.temp.print_unit();
        let mut d;
        let d_str = app.units.density.print_unit();
        let mm;
        let mm_str = "g/mol";
        let mut u;
        let energy_str = app.units.energy.print_unit();
        let mut h;
        let mut s;
        let entropy_str = app.units.entropy.print_unit();
        let mut cp;
        let mut cv;
        let k;
        let mut w;
        let speed_str = app.units.speed.print_unit();
        let mut g;
        let mut jt;
        let jt_str = app.units.jt_coeff.print_unit();
        if app.use_gerg2008 {
            p = app.gerg_cur_state.p;
            p = units::get_pressure(p, app.units.pressure);
            t = app.gerg_cur_state.t;
            t = units::get_temperature(t, app.units.temp);
            mm = app.gerg_cur_state.mm;
            d = app.gerg_cur_state.d;
            d = units::get_density(d, app.units.density, mm);
            u = app.gerg_cur_state.u;
            u = units::get_energy(u, app.units.energy, mm);
            h = app.gerg_cur_state.h;
            h = units::get_energy(h, app.units.energy, mm);
            s = app.gerg_cur_state.s;
            s = units::get_entropy(s, app.units.entropy, mm);
            cp = app.gerg_cur_state.cp;
            cp = units::get_entropy(cp, app.units.entropy, mm);
            cv = app.gerg_cur_state.cv;
            cv = units::get_entropy(cv, app.units.entropy, mm);
            k = app.gerg_cur_state.kappa;
            w = app.gerg_cur_state.w;
            w = units::get_speed(w, app.units.speed);
            g = app.gerg_cur_state.g;
            g = units::get_energy(g, app.units.energy, mm);
            jt = app.gerg_cur_state.jt;
            jt = units::get_jt_coeff(jt, app.units.jt_coeff);
        } else {
            p = app.aga8_cur_state.p;
            p = units::get_pressure(p, app.units.pressure);
            t = app.aga8_cur_state.t;
            t = units::get_temperature(t, app.units.temp);
            mm = app.aga8_cur_state.mm;
            d = app.aga8_cur_state.d;
            d = units::get_density(d, app.units.density, mm);
            u = app.aga8_cur_state.u;
            u = units::get_energy(u, app.units.energy, mm);
            h = app.aga8_cur_state.h;
            h = units::get_energy(h, app.units.energy, mm);
            s = app.aga8_cur_state.s;
            s = units::get_entropy(s, app.units.entropy, mm);
            cp = app.aga8_cur_state.cp;
            cp = units::get_entropy(cp, app.units.entropy, mm);
            cv = app.aga8_cur_state.cv;
            cv = units::get_entropy(cv, app.units.entropy, mm);
            k = app.aga8_cur_state.kappa;
            w = app.aga8_cur_state.w;
            w = units::get_speed(w, app.units.speed);
            g = app.aga8_cur_state.g;
            g = units::get_energy(g, app.units.energy, mm);
            jt = app.aga8_cur_state.jt;
            jt = units::get_jt_coeff(jt, app.units.jt_coeff);
        }

        let items = vec![
        ListItem::new(format!("{:<18}", "Air")).bg(Color::Blue),
        ListItem::new(format!("{:<18} {:.4} {}", "Pressure:", p, p_str)),
        ListItem::new(format!("{:<18} {:.4} {}", "Temperature:", t, t_str)).bg(Color::DarkGray),
        ListItem::new(format!("{:<18} {:.4} {}", "Density:", d, d_str)),
        ListItem::new(format!("{:<18} {:.4} {}", "Molar Mass:", mm, mm_str)).bg(Color::DarkGray),
        ListItem::new(format!("{:<18} {:.4} {}", "Internal Energy:", u, energy_str)),
        ListItem::new(format!("{:<18} {:.4} {}", "Enthalpy:", h, energy_str)).bg(Color::DarkGray),
        ListItem::new(format!("{:<18} {:.4} {}", "Entropy:", s, entropy_str)),
        ListItem::new(format!("{:<18} {:.4} {}", "Cp:", cp, entropy_str)).bg(Color::DarkGray),
        ListItem::new(format!("{:<18} {:.4} {}", "Cv:", cv, entropy_str)),
        ListItem::new(format!("{:<18} {:.4} {}", "Cp/Cv (k):", k, "[]")).bg(Color::DarkGray),
        ListItem::new(format!("{:<18} {:.4} {}", "Speed of Sound:", w, speed_str)),
        ListItem::new(format!("{:<18} {:.4} {}", "Gibbs Energy:", g, energy_str)).bg(Color::DarkGray),
        ListItem::new(format!("{:<18} {:.4} {}", "JT Coeff:", jt, jt_str))

    ];
    items
}
