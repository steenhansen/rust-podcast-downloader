use ratatui::prelude::*;

use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};

pub type TuiTerm = Terminal<CrosstermBackend<Stdout>>;
use crate::file_log;

pub fn init(debug_file: &str) -> io::Result<TuiTerm> {
    let _handle = file_log::do_log(debug_file);
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), EnableMouseCapture)?;

    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore(mut the_tui: TuiTerm, res: io::Result<()>) -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    execute!(stdout(), DisableMouseCapture)?;
    disable_raw_mode()?;
    execute!(the_tui.backend_mut())?;
    the_tui.show_cursor()?;
    if let Err(err) = res {
        println!("Program Error: {err:?}");
    }
    Ok(())
}
