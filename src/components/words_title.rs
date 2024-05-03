#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::consts_globals;
use crate::consts::consts_rects;
use crate::state::state_app;

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn title_show(
    console_frame: &mut Frame,
    the_app: &mut state_app::DownApp,
    app_dim: bool,
    is_downloading_paused: bool,
) {
    let the_title = "Terminal Rust Podcast Downloader";
    let title_area = consts_rects::rect_title(console_frame, the_title);
    let mut wait_color = consts_globals::TITLE_COLOR;
    if is_downloading_paused {
        wait_color = consts_globals::PAUSE_COLOR;
    } else if app_dim {
        wait_color = consts_globals::DIMMED_BACKGROUND_WAIT;
    }
    title_render(
        console_frame,
        title_area,
        the_app,
        "Terminal Rust Podcast Downloader",
        wait_color,
    );
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