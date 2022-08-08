

use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
    widgets::{Widget, Block, Borders, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction},
    Frame,
    style::{Color, Style, Modifier}
};

use std::io;
use crossterm::{
    self,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use edit::edit_file;

use std::error::Error;
use std::cmp::{min, max};

pub struct Location {
    pub line_nr: usize,
    pub file_path: String
}


pub struct MatchItem {
    pub location: Location,
    pub match_text: String
}

pub struct App {
    items: Vec<MatchItem>,
    list_state: ListState,
}


impl App {
    fn up(&mut self) {
        self.list_state.select(
            Some(
                match self.list_state.selected().unwrap() {
                    0 => 0,
                    a => a - 1,
                }
            )
        );
    }

    fn down(&mut self) {
        let curr = self.list_state.selected().unwrap();
        self.list_state.select(
            Some(
                min(self.items.len() - 1, curr + 1)
            )
        );
    }


    pub fn new(items: Vec<MatchItem>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        App{
            items: items,
            list_state: state
        }
    }

    fn open_file(
        &mut self, 
    ) -> Result<(), Box<dyn Error>> {

        edit_file("hello.py");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>>{
        enable_raw_mode()?;

        let mut stdout = io::stdout();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        self.draw(&mut terminal)?;

        info!("Entering main loop");

        loop {
            let message = crossterm::event::read();

            match message.unwrap() {
                Event::Key(key_event) => {
                    if key_event.modifiers.is_empty() {
                        match key_event.code {
                            KeyCode::Up => {
                                info!("Pressed UP");
                                self.up();
                            },
                            KeyCode::Down => {
                                info!("Pressed Down");
                                self.down();
                            },
                            KeyCode::Enter => {
                                info!("Pressed Enter");

                                self.open_file();

                                let mut stdout = io::stdout();
                                execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                                terminal.clear();



                            },
                            _ => {}
                        }
                        self.draw(&mut terminal)?;

                    } else if key_event.modifiers == KeyModifiers::CONTROL {
                        break;
                    }
                }
                Event::Resize(_width, _height) => {
                    self.draw(&mut terminal)?;
                }
                _ => {}
            }
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

    fn draw<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        term.draw(|mut f| {

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref())
                .split(f.size());

            let (filechunk, codechunk) = (chunks[0], chunks[1]);

            let fileblock = Block::default()
                .title("Files")
                .borders(Borders::ALL);

            let files: Vec<ListItem> = self.items.iter()
                .map(|i| 
                    ListItem::new(i.match_text.clone())
                )
                .collect();

            let file_list = List::new(files)
                .block(fileblock)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            f.render_stateful_widget(file_list, filechunk, &mut self.list_state);


            let codeblock = Block::default()
                .title("Code")
                .borders(Borders::ALL);

            f.render_widget(codeblock, chunks[1]);
        })?;

        Ok(())
    }
}



   
