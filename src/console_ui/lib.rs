//! ![Demo](https://github.com/ratatui-org/ratatui/blob/1d39444e3dea6f309cf9035be2417ac711c1abc9/examples/demo2-destroy.gif?raw=true)
//!
//! <div align="center">
//!
//! [![Crate Badge]][Crate] [![Docs Badge]][API Docs] [![CI Badge]][CI Workflow] [![License
//! Badge]](./LICENSE) [![Sponsors Badge]][GitHub Sponsors]<br>
//! [![Codecov Badge]][Codecov] [![Deps.rs Badge]][Deps.rs] [![Discord Badge]][Discord Server]
//! [![Matrix Badge]][Matrix]<br>
//!
//! [Ratatui Website] · [API Docs] · [Examples] · [Changelog] · [Breaking Changes]<br>
//! [Contributing] · [Report a bug] · [Request a Feature] · [Create a Pull Request]
//!
//! </div>
//!
//! # Ratatui
//!
//! [Ratatui][Ratatui Website] is a crate for cooking up terminal user interfaces in Rust. It is a
//! lightweight library that provides a set of widgets and utilities to build complex Rust TUIs.
//! Ratatui was forked from the [tui-rs] crate in 2023 in order to continue its development.
//!
//! ## Installation
//!
//! Add `ratatui` and `crossterm` as dependencies to your cargo.toml:
//!
//! ```shell
//! cargo add ratatui crossterm
//! ```
//!
//! Ratatui uses [Crossterm] by default as it works on most platforms. See the [Installation]
//! section of the [Ratatui Website] for more details on how to use other backends ([Termion] /
//! [Termwiz]).
//!
//! ## Introduction
//!
//! Ratatui is based on the principle of immediate rendering with intermediate buffers. This means
//! that for each frame, your app must render all widgets that are supposed to be part of the UI.
//! This is in contrast to the retained mode style of rendering where widgets are updated and then
//! automatically redrawn on the next frame. See the [Rendering] section of the [Ratatui Website]
//! for more info.
//!
//! You can also watch the [FOSDEM 2024 talk] about Ratatui which gives a brief introduction to
//! terminal user interfaces and showcases the features of Ratatui, along with a hello world demo.
//!
//! ## Other documentation
//!
//! - [Ratatui Website] - explains the library's concepts and provides step-by-step tutorials
//! - [API Docs] - the full API documentation for the library on docs.rs.
//! - [Examples] - a collection of examples that demonstrate how to use the library.
//! - [Contributing] - Please read this if you are interested in contributing to the project.
//! - [Changelog] - generated by [git-cliff] utilizing [Conventional Commits].
//! - [Breaking Changes] - a list of breaking changes in the library.
//!
//! ## Quickstart
//!
//! The following example demonstrates the minimal amount of code necessary to setup a terminal and
//! render "Hello World!". The full code for this example which contains a little more detail is in
//! the [Examples] directory. For more guidance on different ways to structure your application see
//! the [Application Patterns] and [Hello World tutorial] sections in the [Ratatui Website] and the
//! various [Examples]. There are also several starter templates available in the [templates]
//! repository.
//!
//! Every application built with `ratatui` needs to implement the following steps:
//!
//! - Initialize the terminal
//! - A main loop to:
//!   - Handle input events
//!   - Draw the UI
//! - Restore the terminal state
//!
//! The library contains a [`prelude`] module that re-exports the most commonly used traits and
//! types for convenience. Most examples in the documentation will use this instead of showing the
//! full path of each type.
//!
//! ### Initialize and restore the terminal
//!
//! The [`Terminal`] type is the main entry point for any Ratatui application. It is a light
//! abstraction over a choice of [`Backend`] implementations that provides functionality to draw
//! each frame, clear the screen, hide the cursor, etc. It is parametrized over any type that
//! implements the [`Backend`] trait which has implementations for [Crossterm], [Termion] and
//! [Termwiz].
//!
//! Most applications should enter the Alternate Screen when starting and leave it when exiting and
//! also enable raw mode to disable line buffering and enable reading key events. See the [`backend`
//! module] and the [Backends] section of the [Ratatui Website] for more info.
//!
//! ### Drawing the UI
//!
//! The drawing logic is delegated to a closure that takes a [`Frame`] instance as argument. The
//! [`Frame`] provides the size of the area to draw to and allows the app to render any [`Widget`]
//! using the provided [`render_widget`] method. After this closure returns, a diff is performed and
//! only the changes are drawn to the terminal. See the [Widgets] section of the [Ratatui Website]
//! for more info.
//!
//! ### Handling events
//!
//! Ratatui does not include any input handling. Instead event handling can be implemented by
//! calling backend library methods directly. See the [Handling Events] section of the [Ratatui
//! Website] for more info. For example, if you are using [Crossterm], you can use the
//! [`crossterm::event`] module to handle events.
//!
//! ### Example
//!
//! ```rust,no_run
//! use std::io::{self, stdout};
//!
//! use crossterm::{
//!     event::{self, Event, KeyCode},
//!     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//!     ExecutableCommand,
//! };
//! use ratatui::{prelude::*, widgets::*};
//!
//! fn main() -> io::Result<()> {
//!     enable_raw_mode()?;
//!     stdout().execute(EnterAlternateScreen)?;
//!     let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
//!
//!     let mut should_quit = false;
//!     while !should_quit {
//!         terminal.draw(ui)?;
//!         should_quit = handle_events()?;
//!     }
//!
//!     disable_raw_mode()?;
//!     stdout().execute(LeaveAlternateScreen)?;
//!     Ok(())
//! }
//!
//! fn handle_events() -> io::Result<bool> {
//!     if event::poll(std::time::Duration::from_millis(50))? {
//!         if let Event::Key(key) = event::read()? {
//!             if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
//!                 return Ok(true);
//!             }
//!         }
//!     }
//!     Ok(false)
//! }
//!
//! fn ui(frame: &mut Frame) {
//!     frame.render_widget(
//!         Paragraph::new("Hello World!")
//!             .block(Block::default().title("Greeting").borders(Borders::ALL)),
//!         frame.size(),
//!     );
//! }
//! ```
//!
//! Running this example produces the following output:
//!
//! ![docsrs-hello]
//!
//! ## Layout
//!
//! The library comes with a basic yet useful layout management object called [`Layout`] which
//! allows you to split the available space into multiple areas and then render widgets in each
//! area. This lets you describe a responsive terminal UI by nesting layouts. See the [Layout]
//! section of the [Ratatui Website] for more info.
//!
//! ```rust,no_run
//! use ratatui::{prelude::*, widgets::*};
//!
//! fn ui(frame: &mut Frame) {
//!     let main_layout = Layout::new(
//!         Direction::Vertical,
//!         [
//!             Constraint::Length(1),
//!             Constraint::Min(0),
//!             Constraint::Length(1),
//!         ],
//!     )
//!     .split(frame.size());
//!     frame.render_widget(
//!         Block::new().borders(Borders::TOP).title("Title Bar"),
//!         main_layout[0],
//!     );
//!     frame.render_widget(
//!         Block::new().borders(Borders::TOP).title("Status Bar"),
//!         main_layout[2],
//!     );
//!
//!     let inner_layout = Layout::new(
//!         Direction::Horizontal,
//!         [Constraint::Percentage(50), Constraint::Percentage(50)],
//!     )
//!     .split(main_layout[1]);
//!     frame.render_widget(
//!         Block::default().borders(Borders::ALL).title("Left"),
//!         inner_layout[0],
//!     );
//!     frame.render_widget(
//!         Block::default().borders(Borders::ALL).title("Right"),
//!         inner_layout[1],
//!     );
//! }
//! ```
//!
//! Running this example produces the following output:
//!
//! ![docsrs-layout]
//!
//! ## Text and styling
//!
//! The [`Text`], [`Line`] and [`Span`] types are the building blocks of the library and are used in
//! many places. [`Text`] is a list of [`Line`]s and a [`Line`] is a list of [`Span`]s. A [`Span`]
//! is a string with a specific style.
//!
//! The [`style` module] provides types that represent the various styling options. The most
//! important one is [`Style`] which represents the foreground and background colors and the text
//! attributes of a [`Span`]. The [`style` module] also provides a [`Stylize`] trait that allows
//! short-hand syntax to apply a style to widgets and text. See the [Styling Text] section of the
//! [Ratatui Website] for more info.
//!
//! ```rust,no_run
//! use ratatui::{prelude::*, widgets::*};
//!
//! fn ui(frame: &mut Frame) {
//!     let areas = Layout::new(
//!         Direction::Vertical,
//!         [
//!             Constraint::Length(1),
//!             Constraint::Length(1),
//!             Constraint::Length(1),
//!             Constraint::Length(1),
//!             Constraint::Min(0),
//!         ],
//!     )
//!     .split(frame.size());
//!
//!     let span1 = Span::raw("Hello ");
//!     let span2 = Span::styled(
//!         "World",
//!         Style::new()
//!             .fg(Color::Green)
//!             .bg(Color::White)
//!             .add_modifier(Modifier::BOLD),
//!     );
//!     let span3 = "!".red().on_light_yellow().italic();
//!
//!     let line = Line::from(vec![span1, span2, span3]);
//!     let text: Text = Text::from(vec![line]);
//!
//!     frame.render_widget(Paragraph::new(text), areas[0]);
//!     // or using the short-hand syntax and implicit conversions
//!     frame.render_widget(
//!         Paragraph::new("Hello World!".red().on_white().bold()),
//!         areas[1],
//!     );
//!
//!     // to style the whole widget instead of just the text
//!     frame.render_widget(
//!         Paragraph::new("Hello World!").style(Style::new().red().on_white()),
//!         areas[2],
//!     );
//!     // or using the short-hand syntax
//!     frame.render_widget(Paragraph::new("Hello World!").blue().on_yellow(), areas[3]);
//! }
//! ```
//!
//! Running this example produces the following output:
//!
//! ![docsrs-styling]
#![cfg_attr(feature = "document-features", doc = "\n## Features")]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
#![cfg_attr(
    feature = "document-features",
    doc = "[`CrossTermBackend`]: backend::CrosstermBackend"
)]

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod text;
pub mod widgets;

#[doc(inline)]
pub use self::terminal::{CompletedFrame, Frame, Terminal, TerminalOptions, Viewport};

pub mod prelude;
