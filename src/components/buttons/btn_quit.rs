#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_colors;
use crate::misc::misc_ui;
use crate::state::state_app;

use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};
use std::time::SystemTime;

pub fn quit_hover(
    the_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    hover_event: MouseEvent,
) {
    let column = hover_event.column;
    let row = hover_event.row;
    let quit_area = quit_area(the_frame);
    if misc_ui::rect_point_in(column, row, quit_area) {
        the_app.hover_element = state_app::HOVER_X_QUIT.to_string();
    }
}

pub fn quit_render(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    draw_area: Rect,
    spinner: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let mut the_style = Style::default().fg(const_colors::BTN_X_BORDER_READY);
    if the_app.hover_element == state_app::HOVER_X_QUIT {
        the_style = Style::default().fg(const_colors::BTN_X_BORDER_HOVER);
    }
    let paragraph = Block::default().borders(Borders::ALL).style(the_style);
    console_frame.render_widget(paragraph, area_safe);

    let white_square = Rect {
        x: area_safe.x + 2,
        y: area_safe.y + 1,
        width: 1,
        height: 1,
    };
    let white_safe = white_square.intersection(console_frame.size());
    let mut box_style = Style::default().fg(spinner);

    if the_app.hover_element == state_app::HOVER_X_QUIT {
        box_style = Style::new().white().on_black();
    }

    let paragraph = Paragraph::new("X").style(box_style);
    console_frame.render_widget(paragraph, white_safe);
}

pub fn quit_clicked(the_click: MouseEvent, the_frame: &mut Frame) -> bool {
    let column = the_click.column;
    let row = the_click.row;
    let quit_area = quit_area(the_frame);
    if misc_ui::rect_point_in(column, row, quit_area) {
        return true;
    }
    false
}

pub fn quit_show(console_frame: &mut Frame, the_app: &mut state_app::DownApp) {
    let up_right_area = quit_area(console_frame);

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system-time-err");
    let elapsed_sed = now.as_secs();
    let show_alive = elapsed_sed % 5;
    let color_spinner = match show_alive {
        0 => Color::Green,
        1 => Color::LightRed,
        2 => Color::Yellow,
        3 => Color::LightMagenta,
        _ => Color::Cyan,
    };
    quit_render(console_frame, the_app, up_right_area, color_spinner);
}

pub fn quit_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();

    let mut quit_left = 0;
    if area_frame.width > 5 {
        quit_left = area_frame.width - 5;
    }

    let up_right_area = Rect {
        x: quit_left,
        y: 0,
        width: 5,
        height: 3,
    };
    up_right_area
}
