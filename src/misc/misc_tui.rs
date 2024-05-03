#[allow(unused)]
use log::{debug, info, trace, warn};

use crate::consts::const_globals;
use crate::files::file_log;

use crossterm::style::Print;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};
use ratatui::prelude::*;
use std::io::{self, stdout, Stdout};

pub type TuiTerm = Terminal<CrosstermBackend<Stdout>>;

pub fn tui_init(debug_file: &str) -> io::Result<TuiTerm> {
    let _handle = file_log::log_initialize(debug_file);
    execute!(stdout(), Print(const_globals::CLEAR_SCREEN)).expect("print-err");

    enable_raw_mode().expect("tui-enter-raw-err");
    execute!(stdout(), EnterAlternateScreen).expect("tui-enter-alternate-err");
    execute!(stdout(), EnableMouseCapture).expect("tui-mouse-capture-err");

    let crossterm_terminal = Terminal::new(CrosstermBackend::new(stdout()));
    crossterm_terminal
}

pub fn tui_restore(mut the_tui: TuiTerm, res: io::Result<()>) -> io::Result<()> {
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
