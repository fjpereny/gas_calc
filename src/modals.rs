
use ratatui::{
    layout::{
        Constraint, 
        Layout, 
        Rect
    }, 
    style::{
        Color, 
        Style, 
        Stylize
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

use crate::{
    App,
    units::PrintUnit, 
};


pub fn popup_area(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
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

pub fn pressure_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title("Current State Pressure")
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content = Paragraph::new(format!("Enter Pressure {} (press U to change units)\n{}", app.units.pressure.print_unit(), app.input_text.lines()[0]))
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn temperature_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title("Current State Temperature")
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content = Paragraph::new(format!("Enter Temperature {} (press U to change units)\n{}", app.units.temp.print_unit(), app.input_text.lines()[0]))
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn flow_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title("Flow Rate")
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content = Paragraph::new(
        format!("Enter flow rate {} (pressure U to change unit)\n{}", 
        app.units.flow.print_unit(), 
        app.input_text.lines()[0]))
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn gas_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Gas Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content = Paragraph::new(
        format!("Select Gas\n1-Air 2-Ar 3-CO 4-CO2 5-He 6-H2 7-N2 8-O2")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn select_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Change Units"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Unit Type\n1-Pressure  2-Temperature  3-Density  4-Energy  5-Entropy  6-Speed  7-Flow")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn pressure_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Pressure Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Pressure Unit\n1-kPa 2-Bar 3-PSI")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn temperature_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Temperature Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Pressure Unit\n1-K   2-C   3-R   4-F")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn density_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Density Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Density Unit\n1-mol/l   2-kg/m^3   3-lbm/ft^3")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn energy_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Energy Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Energy Unit\n1-J/mol   2-kJ/kg   3-BTU/lbm")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn entropy_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Entropy Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Energy Unit\n1-J/(mol-K)   2-kJ/(kg-K)   3-BTU/(lbm-R)")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}

pub fn speed_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Speed Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select Speed Unit\n1-m/s   2-ft/s")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}


pub fn flow_units_modal(app: &mut App, frame: &mut Frame, main_area: Rect) {
    let modal_width_percent = 60;
    let modal_height_percent = 20;
    let modal_area = popup_area(main_area, modal_width_percent, modal_height_percent);

    // Clear the background behind the modal
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::new()
    .title(format!("Flow Unit Options"))
    .borders(Borders::ALL)
    .style(Style::new().fg(Color::White).bg(Color::Blue));

    let modal_content: Paragraph<'_> = Paragraph::new(
        format!("Select SpeFlowed Unit\n1-kg/s 2-kg/min 3-kg/hr 4-lbm/s 5-lbm/min 6-lbm/hr 7-Nm3/hr 8-SCFM 9-SCFH")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}