mod calculations;
mod gas;
mod modals;
mod units;

use std::os::linux::raw::stat;

use aga8::detail::Detail;
use aga8::gerg2008::Gerg2008;
use aga8::composition::Composition;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use ratatui::{
    layout::{
        Constraint, 
        Layout, 
        Rect
    }, 
    style::{
        Color, Style, Styled, Stylize
    }, 
    widgets::{
        Block, 
        Borders, 
        Clear, 
        List, 
        ListItem, 
        Paragraph,
    }, 
    Frame
};
use ratatui_textarea::TextArea;

use crate::gas::{
    get_gas_comp, 
    set_gas
};
use crate::units::
{
    Units, 
    PrintUnit
};
use crate::calculations::run_calculations;

pub struct App {
    pub pressure_modal_visible: bool,
    pub temperature_modal_visible: bool,
    pub gas_modal_visible: bool,
    pub flow_modal_visible: bool,
    pub select_unit_modal_visible: bool,
    pub pressure_units_modal_visible: bool,
    pub temperature_units_modal_visible: bool,
    pub density_units_modal_visible: bool,
    pub energy_units_modal_visible: bool,
    pub entropy_units_modal_visible: bool,
    pub speed_units_modal_visible: bool,
    pub flow_units_modal_visible: bool,
    pub input_speed_modal_visible: bool,
    pub gear_ratio_modal_visible: bool,
    pub wheel_diameter_modal_visible: bool,
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
    pub input_text: TextArea<'static>,
    pub input_modal_active: bool,
    pub flow_val: f64,
    pub input_speed: f64,
    pub gear_ratio: f64,
    pub wheel_diameter: f64,
    pub gas_text: &'static str,
    pub stp_60_F: bool,
}

impl Default for App {
    fn default() -> Self {
        App { 
            pressure_modal_visible: false,
            temperature_modal_visible: false,
            gas_modal_visible: false,
            flow_modal_visible: false,
            select_unit_modal_visible: false,
            pressure_units_modal_visible: false,
            temperature_units_modal_visible: false,
            density_units_modal_visible: false,
            energy_units_modal_visible: false,
            entropy_units_modal_visible: false,
            speed_units_modal_visible: false,
            flow_units_modal_visible: false,
            input_speed_modal_visible: false,
            gear_ratio_modal_visible: false,
            wheel_diameter_modal_visible: false,
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
            input_text: TextArea::default(),
            input_modal_active: false,
            flow_val: 0.0,
            input_speed: 0.0,
            gear_ratio: 0.0,
            wheel_diameter: 0.0,
            gas_text: "Air",
            stp_60_F: true,
        }
    }
}

fn app_setup(app: &mut App) {
    app.gas_comp = get_gas_comp(gas::Gas::Air);
    app.aga8_cur_state.set_composition(&app.gas_comp);
    app.gerg_cur_state.set_composition(&app.gas_comp);
    app.aga8_inlet_state.set_composition(&app.gas_comp);
    app.gerg_inlet_state.set_composition(&app.gas_comp);
    app.aga8_outlet_state.set_composition(&app.gas_comp);
    app.gerg_outlet_state.set_composition(&app.gas_comp);

    let p = 14.696;
    let p = units::set_pressure(p, units::Pressure::PSI);
    let t = 60.0;
    let t = units::set_temperature(t, units::Temperature::F);

    app.aga8_cur_state.p = p;
    app.gerg_cur_state.p = p;
    app.aga8_inlet_state.p = p;
    app.gerg_inlet_state.p = p;
    app.aga8_outlet_state.p = p;
    app.gerg_outlet_state.p = p;

    app.aga8_cur_state.t = t;
    app.gerg_cur_state.t = t;
    app.aga8_inlet_state.t = t;
    app.gerg_inlet_state.t = t;
    app.aga8_outlet_state.t = t;
    app.gerg_outlet_state.t = t;

    app.aga8_cur_state.density();
    app.gerg_cur_state.density(0);
    app.aga8_cur_state.properties();
    app.gerg_cur_state.properties();

    app.aga8_inlet_state.density();
    app.gerg_inlet_state.density(0);
    app.aga8_inlet_state.properties();
    app.gerg_inlet_state.properties();

    app.aga8_outlet_state.density();
    app.gerg_outlet_state.density(0);
    app.aga8_outlet_state.properties();
    app.gerg_outlet_state.properties();
}


fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();   
    app_setup(&mut app);
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

fn hotkey_menu(app: &mut App) -> Paragraph<'static> {
    let mode_text;
    if app.use_gerg2008 {
        mode_text = "AGA8"
    } else {
        mode_text = "GERG-2008"
    }
    Paragraph::new(
        format!("Esc-Settings\tP-Pressure\tT-Temperature\tU-Change Units\tI-Set Inlet\tO- Set Outlet\tC-Clear\tM-Switch AGA8/GERG")
    )
}

fn draw(frame: &mut Frame, app: &mut App) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Length(16), Fill(1), Length(3)]);
    let [title_area, main_area, calc_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 3]);
    let [left_area, center_area, right_area] = horizontal.areas(main_area);
    let [left_calc_area, center_calc_area, right_calc_area] = horizontal.areas(calc_area);
    
    let gas_mode;
    if app.use_gerg2008 {
        gas_mode = "GERG-2008"
    } else {
        gas_mode = "AGA8"
    }
    frame.render_widget(Block::bordered().title(format!("Thermodynamic Gas Calculator - {}", gas_mode)).style(Color::LightCyan), title_area);
    
    let hotkey_par = hotkey_menu(app)
        .block(
            Block::bordered()
            .title("Hotkeys")
            .style(Color::LightCyan)
        );
    frame.render_widget(hotkey_par, status_area);
    
    let items = get_gas_properties(app, GasState::Current);
    let items_list = List::new(items)
    .block(Block::bordered()
    .title("Current State"));
    frame.render_widget(items_list, left_area);

    if app.show_inlet_state {
        let items = get_gas_properties(app, GasState::Inlet);
        let items_list = List::new(items)
        .block(Block::bordered()
        .title("Inlet State")
        .style(Color::Green)
    );
    frame.render_widget(items_list, center_area);
} else {
    frame.render_widget(
        Block::bordered().title("Inlet State (press I to set)")
        .style(Color::Red), 
        center_area);
    }
    
    if app.show_outlet_state {
        let items = get_gas_properties(app, GasState::Outlet);
        let items_list = List::new(items)
            .block(Block::bordered()
            .title("Outlet State")
            .style(Color::Green)
        );
        frame.render_widget(items_list, right_area);
    } else {
        frame.render_widget(
            Block::bordered().title("Outlet State (press O to set)")
            .style(Color::Red), 
            right_area);
    }
        
    if app.show_inlet_state && app.show_outlet_state {
        let pr = calculations::pressure_ratio(app);
        let title_text;
        if pr < 1.0 {
            title_text = "Expansion";
        } else if pr > 1.0 {
            title_text = "Compression";
        } else {
            title_text = "Isobaric";
        }
        let [state_change, isentropic_calcs, comp_calcs] = run_calculations(app);
        let items_list = List::new(state_change)
        .block(Block::bordered()
        .title(format!("State Change ({})", title_text))
        .style(Color::LightCyan)
        );
        frame.render_widget(items_list, center_calc_area);
        let items_list = List::new(isentropic_calcs)
        .block(Block::bordered()
        .title(format!("Isentropic Calculations"))
        .style(Color::LightCyan)
        );
        frame.render_widget(items_list, right_calc_area);
        let items_list = List::new(comp_calcs)
        .block(Block::bordered()
        .title(format!("Dimensionless Data"))
        .style(Color::LightYellow)
        );
        frame.render_widget(items_list, left_calc_area);
    } else {
        let [_, _, comp_calcs] = run_calculations(app);
        let items_list = Block::bordered()
            .set_style(Style::default().fg(Color::Red))
            .title(format!("State Change")
        );
        frame.render_widget(items_list, center_calc_area);
        let items_list = Block::bordered()
            .set_style(Style::default().fg(Color::Red))
            .title(format!("Isentropic Calculations")
        );
        frame.render_widget(items_list, right_calc_area);
        let items_list = List::new(comp_calcs)
            .block(Block::bordered()
            .title(format!("Dimensionless Data"))
            .style(Color::LightYellow).fg(Color::Yellow)
        );
        frame.render_widget(items_list, left_calc_area)
    }

    if app.select_unit_modal_visible {
        modals::select_units_modal(app, frame, main_area);
    }
    if app.pressure_modal_visible { 
        app.input_modal_active = true;
        modals::pressure_modal(app, frame, main_area);
    }
    if app.pressure_units_modal_visible {
        modals::pressure_units_modal(app, frame, main_area);
    }
    if app.temperature_modal_visible { 
        app.input_modal_active = true;
        modals::temperature_modal(app, frame, main_area);
    }
    if app.temperature_units_modal_visible {
        modals::temperature_units_modal(app, frame, main_area);
    }
    if app.density_units_modal_visible { 
        modals::density_units_modal(app, frame, main_area);
    }
    if app.energy_units_modal_visible { 
        modals::energy_units_modal(app, frame, main_area);
    }
    if app.entropy_units_modal_visible { 
        modals::entropy_units_modal(app, frame, main_area);
    }
    if app.speed_units_modal_visible { 
        modals::speed_units_modal(app, frame, main_area);
    }
    if app.gas_modal_visible {
        modals::gas_modal(app, frame, main_area);
    }
    if app.flow_modal_visible {
        app.input_modal_active = true;
        modals::flow_modal(app, frame, main_area);
    }
    if app.flow_units_modal_visible {
        modals::flow_units_modal(app, frame, main_area);
    }
    if app.input_speed_modal_visible {
        modals::input_speed_modal(app, frame, main_area);
    }
    if app.gear_ratio_modal_visible {
        modals::gear_ratio_modal(app, frame, main_area);
    }
    if app.wheel_diameter_modal_visible {
        modals::wheel_diameter_modal(app, frame, main_area);
    }
}

fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if app.input_modal_active {
        match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Enter => {
                let input = app.input_text.lines()[0].trim();
                let parse = input.parse::<f64>();
                if parse.is_ok() {
                    let val = parse.unwrap();
                    if app.pressure_modal_visible {
                        set_cur_pressure(val, app);
                    } else if app.temperature_modal_visible {
                        set_cur_temperature(val, app);
                    } else if app.flow_modal_visible {
                        set_cur_flow(val, app);
                    } else if app.input_speed_modal_visible {
                        app.input_speed = val;
                    } else if app.gear_ratio_modal_visible {
                        app.gear_ratio = val;
                    } else if app.wheel_diameter_modal_visible {
                        app.wheel_diameter = val;
                    }
                }
                app.input_modal_active = false;
                app.temperature_modal_visible = false;
                app.pressure_modal_visible = false;
                app.flow_modal_visible = false;
                app.input_speed_modal_visible = false;
                app.gear_ratio_modal_visible = false;
                app.wheel_diameter_modal_visible = false;
                app.input_text = TextArea::default();
            },
            KeyCode::Esc => {
                app.input_modal_active = false;
                app.pressure_modal_visible = false;
                app.temperature_modal_visible = false;
                app.flow_modal_visible = false;
                app.input_speed_modal_visible = false;
                app.gear_ratio_modal_visible = false;
                app.wheel_diameter_modal_visible = false;
                app.input_text = TextArea::default();
            },
            KeyCode::Backspace => {
                app.input_text.delete_char();
            },
            KeyCode::Char('u') => {
                if app.pressure_modal_visible {
                    app.pressure_units_modal_visible = true;
                    app.pressure_modal_visible = false;
                    app.input_modal_active = false;
                } else if app.temperature_modal_visible {
                    app.temperature_units_modal_visible = true;
                    app.temperature_modal_visible = false;
                    app.input_modal_active = false;
                } else if app.flow_modal_visible {
                    app.flow_units_modal_visible = true;
                    app.flow_modal_visible = false;
                    app.input_modal_active = false;
                }
            },
            _ =>{
                    let c = key.code.as_char();
                    if c.is_some() {
                        let c = c.unwrap();
                        if c.is_numeric() || c == '.' || c == '-' {
                            app.input_text.insert_char(c);
                        }
                    }
            },
            },
            _ => {}
        }
        Ok(false)
    } else if app.gas_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.gas_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    set_gas(app, get_gas_comp(gas::Gas::Air));
                    app.gas_text = "Air";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    set_gas(app, get_gas_comp(gas::Gas::Argon));
                    app.gas_text = "Argon";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    set_gas(app, get_gas_comp(gas::Gas::CO));
                    app.gas_text = "Carbon Monoxide";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('4') => {
                    set_gas(app, get_gas_comp(gas::Gas::CO2));
                    app.gas_text = "Carbon Dioxide";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('5') => {
                    set_gas(app, get_gas_comp(gas::Gas::Helium));
                    app.gas_text = "Helium";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('6') => {
                    set_gas(app, get_gas_comp(gas::Gas::Hydrogen));
                    app.gas_text = "Hydrogen";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('7') => {
                    set_gas(app, get_gas_comp(gas::Gas::Nitrogen));
                    app.gas_text = "Nitrogen";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('8') => {
                    set_gas(app, get_gas_comp(gas::Gas::Oxygen));
                    app.gas_text = "Oxygen";
                    app.gas_modal_visible = false;
                },
                KeyCode::Char('9') => {
                    app.gas_text = "Custom";
                    app.gas_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.select_unit_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code{
                KeyCode::Enter => {
                    app.select_unit_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.select_unit_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.select_unit_modal_visible = false;
                    app.pressure_units_modal_visible = true;
                },
                KeyCode::Char('2') => {
                    app.select_unit_modal_visible = false;
                    app.temperature_units_modal_visible = true;
                },
                KeyCode::Char('3') => {
                    app.select_unit_modal_visible = false;
                    app.density_units_modal_visible = true;
                },
                KeyCode::Char('4') => {
                    app.select_unit_modal_visible = false;
                    app.energy_units_modal_visible = true;
                },
                KeyCode::Char('5') => {
                    app.select_unit_modal_visible = false;
                    app.entropy_units_modal_visible = true;
                },
                KeyCode::Char('6') => {
                    app.select_unit_modal_visible = false;
                    app.speed_units_modal_visible = true;
                },
                KeyCode::Char('7') => {
                    app.select_unit_modal_visible = false;
                    app.flow_units_modal_visible = true;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.pressure_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.pressure_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.pressure_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.pressure = units::Pressure::kPa;
                    app.pressure_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.pressure = units::Pressure::Bar;
                    app.pressure_units_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    app.units.pressure = units::Pressure::PSI;
                    app.pressure_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.temperature_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.temperature_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.temperature_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.temp = units::Temperature::K;
                    app.temperature_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.temp = units::Temperature::C;
                    app.temperature_units_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    app.units.temp = units::Temperature::R;
                    app.temperature_units_modal_visible = false;
                },
                KeyCode::Char('4') => {
                    app.units.temp = units::Temperature::F;
                    app.temperature_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.density_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.density_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.density_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.density = units::Density::mol_l;
                    app.density_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.density = units::Density::kg_m3;
                    app.density_units_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    app.units.density = units::Density::lbm_ft3;
                    app.density_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.energy_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.energy_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.energy_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.energy = units::Energy::J_mol;
                    app.energy_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.energy = units::Energy::kJ_kg;
                    app.energy_units_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    app.units.energy = units::Energy::BTU_lbm;
                    app.energy_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.entropy_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.entropy_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.entropy_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.entropy = units::Entropy::J_mol_K;
                    app.entropy_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.entropy = units::Entropy::kJ_kg_K;
                    app.entropy_units_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    app.units.entropy = units::Entropy::BTU_lbm_R;
                    app.entropy_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.speed_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.speed_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.speed_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.speed = units::Speed::m_s;
                    app.speed_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.speed = units::Speed::ft_s;
                    app.speed_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else if app.flow_units_modal_visible {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => {
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Esc => {
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('1') => {
                    app.units.flow = units::Flow::kg_s;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('2') => {
                    app.units.flow = units::Flow::kg_m;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('3') => {
                    app.units.flow = units::Flow::kg_h;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('4') => {
                    app.units.flow = units::Flow::lbm_s;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('5') => {
                    app.units.flow = units::Flow::lbm_m;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('6') => {
                    app.units.flow = units::Flow::lbm_h;
                    app.speed_units_modal_visible = false;
                },
                KeyCode::Char('7') => {
                    app.units.flow = units::Flow::Nm3_h;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('8') => {
                    app.units.flow = units::Flow::scfm;
                    app.flow_units_modal_visible = false;
                },
                KeyCode::Char('9') => {
                    app.units.flow = units::Flow::scfh;
                    app.flow_units_modal_visible = false;
                },
                _ =>{},
            },
            _ => {}
        }
        Ok(false)
    } else {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('p') => app.pressure_modal_visible = !app.pressure_modal_visible,
                KeyCode::Char('t') => app.temperature_modal_visible = !app.temperature_modal_visible,
                KeyCode::Char('u') => app.select_unit_modal_visible = !app.select_unit_modal_visible,
                KeyCode::Char('i') => set_inlet_conditions(app),
                KeyCode::Char('o') => set_outlet_conditions(app),
                KeyCode::Char('m') => {
                    app.use_gerg2008 = ! app.use_gerg2008;
                    recalculate(app);
                },
                KeyCode::Char('c') => {
                    app.show_inlet_state = false;
                    app.show_outlet_state = false;
                },
                KeyCode::Char('g') => {
                    app.gas_modal_visible = ! app.gas_modal_visible
                }
                KeyCode::Char('f') => {
                    app.flow_modal_visible = ! app.flow_modal_visible
                }
                KeyCode::Char('s') => {
                    app.input_speed_modal_visible = ! app.input_speed_modal_visible;
                    app.input_modal_active = true;
                }
                KeyCode::Char('r') => {
                    app.gear_ratio_modal_visible = true;
                    app.input_modal_active = true;
                }
                KeyCode::Char('w') => {
                    app.wheel_diameter_modal_visible = true;
                    app.input_modal_active = true;
                }
                _ => {}
            },
            _ => {}
    }
    Ok(false)
    }
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
    recalculate(app);
}

fn set_pressure(app: &mut App, state: GasState) {
    let p_aga8 = units::get_pressure(app.aga8_cur_state.p, app.units.pressure);
    let p_gerg = units::get_pressure(app.gerg_cur_state.p, app.units.pressure);
    match state {
        GasState::Inlet => {
            app.aga8_inlet_state.p = units::set_pressure(p_aga8, app.units.pressure);
            app.gerg_inlet_state.p = units::set_pressure(p_gerg, app.units.pressure);
            recalculate(app);
        }
        GasState::Outlet => {
            app.aga8_outlet_state.p = units::set_pressure(p_aga8, app.units.pressure);
            app.gerg_outlet_state.p = units::set_pressure(p_gerg, app.units.pressure);
            recalculate(app);
        }
        _ => {}
    } 
}

fn get_temperature(app: &mut App) -> f64 {
    if app.use_gerg2008 {
        units::get_temperature(app.gerg_cur_state.t, app.units.temp)
    } else {
        units::get_temperature(app.aga8_cur_state.t, app.units.temp)
    }
}

fn set_cur_temperature(temperature: f64, app: &mut App) {
    let t_val = units::set_temperature(temperature, app.units.temp);
    app.aga8_cur_state.t = t_val;
    app.gerg_cur_state.t = t_val;
    recalculate(app);
}

fn set_cur_flow(flow_rate: f64, app: &mut App) {
    let flow_val = units::set_flow(
        flow_rate, 
        app.units.flow, 
        &app.gas_comp, 
        app.stp_60_F, 
        app.use_gerg2008
    );
    app.flow_val = flow_val;
}

fn set_temperature(app: &mut App, state: GasState) {
    let t_aga8 = units::get_temperature(app.aga8_cur_state.t, app.units.temp);
    let t_gerg = units::get_temperature(app.gerg_cur_state.t, app.units.temp);
    match state {
        GasState::Inlet => {
            app.aga8_inlet_state.t = units::set_temperature(t_aga8, app.units.temp);
            app.gerg_inlet_state.t = units::set_temperature(t_gerg, app.units.temp);
            recalculate(app);
        }
        GasState::Outlet => {
            app.aga8_outlet_state.t = units::set_temperature(t_aga8, app.units.temp);
            app.gerg_outlet_state.t = units::set_temperature(t_gerg, app.units.temp);
            recalculate(app);
        }
        _ => {}
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
    set_pressure(app, GasState::Inlet);
    set_temperature(app, GasState::Inlet);
    recalculate(app);
    app.show_inlet_state = true;
}

fn set_outlet_conditions(app: &mut App) {
    app.aga8_outlet_state.set_composition(&copy_composition(&app.gas_comp));
    app.gerg_outlet_state.set_composition(&copy_composition(&app.gas_comp));
    set_pressure(app, GasState::Outlet);
    set_temperature(app, GasState::Outlet);
    recalculate(app);
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

enum GasState {
    Current,
    Inlet,
    Outlet,
}

fn get_gas_properties(app: &'_ App, state: GasState) -> Vec<ListItem<'_>> {
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
        let z;
        let mut w;
        let speed_str = app.units.speed.print_unit();
        let mut g;
        let mut jt;
        let jt_str = app.units.jt_coeff.print_unit();

        match state {
            GasState::Current => {
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
                    z = app.gerg_cur_state.z;
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
                    z = app.aga8_cur_state.z;
                    w = app.aga8_cur_state.w;
                    w = units::get_speed(w, app.units.speed);
                    g = app.aga8_cur_state.g;
                    g = units::get_energy(g, app.units.energy, mm);
                    jt = app.aga8_cur_state.jt;
                    jt = units::get_jt_coeff(jt, app.units.jt_coeff);
            }
            let items = vec![
                ListItem::new(format!("{:<18}", app.gas_text)).fg(Color::White).bg(Color::Blue),
                ListItem::new(format!("{:<18} {:.4} {}", "Pressure:", p, p_str)).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Temperature:", t, t_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Density:", d, d_str)).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Molar Mass:", mm, mm_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Internal Energy:", u, energy_str)).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Enthalpy:", h, energy_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Entropy:", s, entropy_str)).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Cp:", cp, entropy_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Cv:", cv, entropy_str)).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Cp/Cv (k):", k, "[]")).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Z:", z, "[]")).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Speed of Sound:", w, speed_str)).fg(Color::Black).bg(Color::DarkGray),
                // ListItem::new(format!("{:<18} {:.4} {}", "Gibbs Energy:", g, energy_str)).fg(Color::White).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "JT Coeff:", jt, jt_str)).fg(Color::White).bg(Color::Black),
            ];
                return items
            },
            GasState::Inlet => {
                if app.use_gerg2008 {
                    p = app.gerg_inlet_state.p;
                    p = units::get_pressure(p, app.units.pressure);
                    t = app.gerg_inlet_state.t;
                    t = units::get_temperature(t, app.units.temp);
                    mm = app.gerg_inlet_state.mm;
                    d = app.gerg_inlet_state.d;
                    d = units::get_density(d, app.units.density, mm);
                    u = app.gerg_inlet_state.u;
                    u = units::get_energy(u, app.units.energy, mm);
                    h = app.gerg_inlet_state.h;
                    h = units::get_energy(h, app.units.energy, mm);
                    s = app.gerg_inlet_state.s;
                    s = units::get_entropy(s, app.units.entropy, mm);
                    cp = app.gerg_inlet_state.cp;
                    cp = units::get_entropy(cp, app.units.entropy, mm);
                    cv = app.gerg_inlet_state.cv;
                    cv = units::get_entropy(cv, app.units.entropy, mm);
                    k = app.gerg_inlet_state.kappa;
                    z = app.gerg_inlet_state.z;
                    w = app.gerg_inlet_state.w;
                    w = units::get_speed(w, app.units.speed);
                    g = app.gerg_inlet_state.g;
                    g = units::get_energy(g, app.units.energy, mm);
                    jt = app.gerg_inlet_state.jt;
                    jt = units::get_jt_coeff(jt, app.units.jt_coeff);
                } else {
                    p = app.aga8_inlet_state.p;
                    p = units::get_pressure(p, app.units.pressure);
                    t = app.aga8_inlet_state.t;
                    t = units::get_temperature(t, app.units.temp);
                    mm = app.aga8_inlet_state.mm;
                    d = app.aga8_inlet_state.d;
                    d = units::get_density(d, app.units.density, mm);
                    u = app.aga8_inlet_state.u;
                    u = units::get_energy(u, app.units.energy, mm);
                    h = app.aga8_inlet_state.h;
                    h = units::get_energy(h, app.units.energy, mm);
                    s = app.aga8_inlet_state.s;
                    s = units::get_entropy(s, app.units.entropy, mm);
                    cp = app.aga8_inlet_state.cp;
                    cp = units::get_entropy(cp, app.units.entropy, mm);
                    cv = app.aga8_inlet_state.cv;
                    cv = units::get_entropy(cv, app.units.entropy, mm);
                    k = app.aga8_inlet_state.kappa;
                    z = app.aga8_inlet_state.z;
                    w = app.aga8_inlet_state.w;
                    w = units::get_speed(w, app.units.speed);
                    g = app.aga8_inlet_state.g;
                    g = units::get_energy(g, app.units.energy, mm);
                    jt = app.aga8_inlet_state.jt;
                    jt = units::get_jt_coeff(jt, app.units.jt_coeff);
            }
            let items = vec![
                ListItem::new(format!("{:<18}", app.gas_text)).fg(Color::White).bg(Color::Blue),
                ListItem::new(format!("{:<18} {:.4} {}", "Pressure:", p, p_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Temperature:", t, t_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Density:", d, d_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Molar Mass:", mm, mm_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Internal Energy:", u, energy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Enthalpy:", h, energy_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Entropy:", s, entropy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Cp:", cp, entropy_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Cv:", cv, entropy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Cp/Cv (k):", k, "[]")).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Z:", z, "[]")).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Speed of Sound:", w, speed_str)).fg(Color::Black).bg(Color::DarkGray),
                // ListItem::new(format!("{:<18} {:.4} {}", "Gibbs Energy:", g, energy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "JT Coeff:", jt, jt_str)).fg(Color::Green).bg(Color::Black),
            ];
                return items
            },
            GasState::Outlet => {
                if app.use_gerg2008 {
                    p = app.gerg_outlet_state.p;
                    p = units::get_pressure(p, app.units.pressure);
                    t = app.gerg_outlet_state.t;
                    t = units::get_temperature(t, app.units.temp);
                    mm = app.gerg_outlet_state.mm;
                    d = app.gerg_outlet_state.d;
                    d = units::get_density(d, app.units.density, mm);
                    u = app.gerg_outlet_state.u;
                    u = units::get_energy(u, app.units.energy, mm);
                    h = app.gerg_outlet_state.h;
                    h = units::get_energy(h, app.units.energy, mm);
                    s = app.gerg_outlet_state.s;
                    s = units::get_entropy(s, app.units.entropy, mm);
                    cp = app.gerg_outlet_state.cp;
                    cp = units::get_entropy(cp, app.units.entropy, mm);
                    cv = app.gerg_outlet_state.cv;
                    cv = units::get_entropy(cv, app.units.entropy, mm);
                    k = app.gerg_outlet_state.kappa;
                    z = app.gerg_outlet_state.z;
                    w = app.gerg_outlet_state.w;
                    w = units::get_speed(w, app.units.speed);
                    g = app.gerg_outlet_state.g;
                    g = units::get_energy(g, app.units.energy, mm);
                    jt = app.gerg_outlet_state.jt;
                    jt = units::get_jt_coeff(jt, app.units.jt_coeff);
                } else {
                    p = app.aga8_outlet_state.p;
                    p = units::get_pressure(p, app.units.pressure);
                    t = app.aga8_outlet_state.t;
                    t = units::get_temperature(t, app.units.temp);
                    mm = app.aga8_outlet_state.mm;
                    d = app.aga8_outlet_state.d;
                    d = units::get_density(d, app.units.density, mm);
                    u = app.aga8_outlet_state.u;
                    u = units::get_energy(u, app.units.energy, mm);
                    h = app.aga8_outlet_state.h;
                    h = units::get_energy(h, app.units.energy, mm);
                    s = app.aga8_outlet_state.s;
                    s = units::get_entropy(s, app.units.entropy, mm);
                    cp = app.aga8_outlet_state.cp;
                    cp = units::get_entropy(cp, app.units.entropy, mm);
                    cv = app.aga8_outlet_state.cv;
                    cv = units::get_entropy(cv, app.units.entropy, mm);
                    k = app.aga8_outlet_state.kappa;
                    z = app.aga8_outlet_state.z;
                    w = app.aga8_outlet_state.w;
                    w = units::get_speed(w, app.units.speed);
                    g = app.aga8_outlet_state.g;
                    g = units::get_energy(g, app.units.energy, mm);
                    jt = app.aga8_outlet_state.jt;
                    jt = units::get_jt_coeff(jt, app.units.jt_coeff);
            }
            let items = vec![
                ListItem::new(format!("{:<18}", app.gas_text)).fg(Color::White).bg(Color::Blue),
                ListItem::new(format!("{:<18} {:.4} {}", "Pressure:", p, p_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Temperature:", t, t_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Density:", d, d_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Molar Mass:", mm, mm_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Internal Energy:", u, energy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Enthalpy:", h, energy_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Entropy:", s, entropy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Cp:", cp, entropy_str)).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Cv:", cv, entropy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Cp/Cv (k):", k, "[]")).fg(Color::Black).bg(Color::DarkGray),
                ListItem::new(format!("{:<18} {:.4} {}", "Z:", z, "[]")).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "Speed of Sound:", w, speed_str)).fg(Color::Black).bg(Color::DarkGray),
                // ListItem::new(format!("{:<18} {:.4} {}", "Gibbs Energy:", g, energy_str)).fg(Color::Green).bg(Color::Black),
                ListItem::new(format!("{:<18} {:.4} {}", "JT Coeff:", jt, jt_str)).fg(Color::Green).bg(Color::Black),
            ];
                return items
            }
        }
        
}

fn recalculate(app: &mut App) {
    app.aga8_cur_state.density();
    app.gerg_cur_state.density(0);
    app.aga8_cur_state.properties();
    app.gerg_cur_state.properties();

    app.aga8_inlet_state.density();
    app.gerg_inlet_state.density(0);
    app.aga8_inlet_state.properties();
    app.gerg_inlet_state.properties();

    app.aga8_outlet_state.density();
    app.gerg_outlet_state.density(0);
    app.aga8_outlet_state.properties();
    app.gerg_outlet_state.properties();
}

