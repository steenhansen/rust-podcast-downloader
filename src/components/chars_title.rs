#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::area_rects;
use crate::state::app_state;

use crate::consts::const_globals;

use ratatui::layout::Rect;

use ratatui::{prelude::*, widgets::*};

pub fn show_title(
    console_frame: &mut Frame,
    the_app: &mut app_state::DownApp,
    dim_background: bool,
    is_downloading_paused: bool,
) {
    let the_title = "Terminal Rust Podcast Downloader";
    let title_area = area_rects::get_title_area(console_frame, the_title);
    let mut wait_color = const_globals::TITLE_COLOR;
    if is_downloading_paused {
        wait_color = const_globals::PAUSE_COLOR;
    } else if dim_background {
        wait_color = const_globals::DIMMED_BACKGROUND_WAIT;
    }
    render_title(
        console_frame,
        title_area,
        the_app,
        "Terminal Rust Podcast Downloader",
        wait_color,
    );
}

pub fn render_title(
    console_frame: &mut Frame,
    draw_area: Rect,
    _the_app: &mut app_state::DownApp,
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
