
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

    let modal_content = Paragraph::new(format!("Enter temperature {}\n{}", app.units.temp.print_unit(), app.input_text.lines()[0]))
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
        format!("Select Gas\n1-Air 2-Ar 3-CO 4-CO2 5-He 6-H2 7-N2 8-O2 9-Custom")
    )
    .block(Block::new().padding(ratatui::widgets::Padding::uniform(1)));

    frame.render_widget(modal_block, modal_area);
    frame.render_widget(modal_content, modal_area);
}