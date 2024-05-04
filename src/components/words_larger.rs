#[allow(unused)]
use log::{debug, info, trace, warn};

use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};

pub fn title_show(console_frame: &mut Frame) {
    let the_title = "Increase the size of the window.";
    let title_area = Rect {
        x: 0,
        y: 0,
        width: the_title.len() as u16,
        height: 1,
    };
    let area_safe = title_area.intersection(console_frame.size());
    let paragraph = Paragraph::new(the_title);
    console_frame.render_widget(paragraph, area_safe);
}
