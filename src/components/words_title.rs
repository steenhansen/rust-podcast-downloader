#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_colors;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn rect_title(console_frame: &mut Frame, the_title: &str) -> Rect {
    let area_frame = console_frame.size();
    let title_width = the_title.len() as u16;
    let mut left_start = 0;
    if area_frame.width > title_width {
        left_start = (area_frame.width - title_width) / 2;
    }
    let up_right_area = Rect {
        x: left_start,
        y: 0,
        width: title_width,
        height: 1,
    };
    up_right_area
}

pub fn title_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let the_title = "Terminal Rust Podcast Downloader";
    let title_area = rect_title(console_frame, the_title);
    let mut wait_color = const_colors::TITLE_COLOR;
    if is_downloading_paused {
        wait_color = const_colors::PAUSE_COLOR;
    } else if app_dim {
        wait_color = const_colors::DIMMED_BACKGROUND_WAIT;
    }
    title_render(console_frame, title_area, the_app, the_title, wait_color);
}

pub fn title_render(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut state_app::DownApp,
    box_title: &str,
    title_color: Color,
) {
    let area_safe = draw_area.intersection(console_frame.size());
    let title = Block::new()
        .title_alignment(Alignment::Center)
        .title(box_title)
        .style(
            Style::default()
                .fg(title_color)
                //                .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED),
                .add_modifier(Modifier::ITALIC),
        );
    console_frame.render_widget(title, area_safe);
}
