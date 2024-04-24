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
    enable_raw_mode().expect("tui-enter-raw-err");
    execute!(stdout(), EnterAlternateScreen).expect("tui-enter-alternate-err");
    execute!(stdout(), EnableMouseCapture).expect("tui-mouse-capture-err");

    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore(mut the_tui: TuiTerm, res: io::Result<()>) -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen).expect("tui-leave-alternate-err");
    execute!(stdout(), DisableMouseCapture).expect("tui-mouse-disable-err");
    disable_raw_mode().expect("tui-leave-raw-err");
    execute!(the_tui.backend_mut()).expect("tui-mut-err");
    the_tui.show_cursor().expect("tui-show-err");
    if let Err(err) = res {
        println!("Program Error: {err:?}");
    }
    Ok(())
}
