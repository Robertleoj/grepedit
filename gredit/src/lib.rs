
use simple_log::LogConfigBuilder;
use std::error::Error;
pub mod app;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}



use crossterm::{
    self,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
    widgets::{Widget, Block, Borders, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction},
    Frame,
    style::{Color, Style, Modifier}
};

use crossbeam_channel::{select, tick, unbounded, Receiver};
use std::{io, thread, time::Duration};

#[macro_use]
extern crate simple_log;

pub fn build_log_config() -> Result<(), Box<dyn Error>>{
    let config = LogConfigBuilder::builder()
        .path("./project.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("debug")
        .output_file()
        .build();

    simple_log::new(config)?;
    Ok(())
}



pub fn run() -> Result<(), Box<dyn Error>> {
    let mut app = app::App::new(vec![
        app::MatchItem{
            location: app::Location{
                line_nr: 10, 
                file_path: "hello.py".to_string()
            }, 
            match_text: "def a():".to_string()
        },
        app::MatchItem{
            location: app::Location{
                line_nr: 20, 
                file_path: "index.html".to_string()
            }, 
            match_text: "<def>".to_string()
        },
    ]);

    // let ui_events_receiver = setup_ui_events();

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    app.draw(&mut terminal)?;

    let ticker = tick(Duration::from_secs_f64(0.01));

    info!("Entering main loop");

    loop {
        let message = crossterm::event::read();
        // select! {

            // recv(ui_events_receiver) -> message => {

        match message.unwrap() {
            Event::Key(key_event) => {
                if key_event.modifiers.is_empty() {
                    match key_event.code {
                        KeyCode::Up => {
                            info!("Pressed UP");
                            app.up();
                        },
                        KeyCode::Down => {
                            info!("Pressed Down");
                            app.down();
                        },
                        KeyCode::Enter => {
                            info!("Pressed Enter");

                            app.open_file(&mut terminal)?;

                            let mut stdout = io::stdout();
                            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                            terminal.clear();



                        },
                        _ => {}
                    }
                    app.draw(&mut terminal)?;

                } else if key_event.modifiers == KeyModifiers::CONTROL {
                    break;
                }
            }
            Event::Resize(_width, _height) => {
                app.draw(&mut terminal)?;
            }
            _ => {}
        }
            // }
        // }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}



