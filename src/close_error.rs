use crate::const_areas;
use crate::input_box;
use crate::misc_fun;
use crate::render_app;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::{prelude::*, widgets::*};

pub fn render_pop_up_close(the_app: &mut render_app::DownApp, console_frame: &mut Frame) {
    if the_app.ui_state == the_types::UiState::StateWaitForPopErrorClose {
        draw_close_popup(the_app, console_frame);
    }
}

fn draw_close_popup(the_app: &mut render_app::DownApp, console_frame: &mut Frame) {
    let area = console_frame.size();

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let the_err_mess = the_app.cur_error.clone();
    let paragraph = Paragraph::new(the_err_mess)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("My Error"));
    let area = misc_fun::centered_rect(60, 20, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);
    let pop_close_area = get_pop_close_area(console_frame);
    input_box::render_close(console_frame, pop_close_area, "Ok");
}

pub fn get_pop_close_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let area = misc_fun::centered_rect(60, 20, area_frame);

    let mut close_err_area = area;
    close_err_area.x = close_err_area.x + (close_err_area.width / 2) - 2;
    close_err_area.y = close_err_area.y + (close_err_area.height / 2) - 1;
    close_err_area.width = 5;
    close_err_area.height = 3;
    close_err_area
}

pub fn get_end_prog_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let up_right_area = Rect {
        x: area_frame.width - 5,
        y: 0,
        width: 5,
        height: 3,
    };
    up_right_area
}

pub fn get_episode_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_episodes_area = Rect {
        x: const_areas::START_X_EPISODE,
        y: const_areas::START_Y_EPISODE,
        width: area_frame.width - 10,
        height: area_frame.height - 10,
    };
    elastic_episodes_area
}

pub fn get_podcast_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let elastic_podcasts_area = Rect {
        x: const_areas::START_X_PODCAST,
        y: const_areas::START_Y_PODCAST,
        width: const_areas::WIDTH_PODCAST,
        height: area_frame.height - 10,
    };
    elastic_podcasts_area
}

pub fn get_status_area(console_frame: &mut Frame) -> Rect {
    let area_frame = console_frame.size();
    let status_area = Rect {
        x: 0,
        y: area_frame.height - 1,
        width: area_frame.width,
        height: 1,
    };
    status_area
}
