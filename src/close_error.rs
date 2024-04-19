use crate::app_ui;
use crate::area_rects;
use crate::render_misc;
use crate::the_types;
#[allow(unused)]
use log::{debug, info, trace, warn};
use ratatui::{prelude::*, widgets::*};

pub fn render_pop_up_close(console_frame: &mut Frame, the_app: &mut app_ui::DownApp) {
    if the_app.ui_state == the_types::UiState::StateWaitForPopErrorClose {
        draw_close_popup(console_frame, the_app);
    }
}

fn draw_close_popup(console_frame: &mut Frame, the_app: &mut app_ui::DownApp) {
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
    let area = area_rects::centered_rect(60, 20, area);
    console_frame.render_widget(Clear, area);
    console_frame.render_widget(paragraph, area);
    let pop_close_area = area_rects::get_pop_close_area(console_frame);
    render_misc::render_close(console_frame, pop_close_area, "Ok");
}
