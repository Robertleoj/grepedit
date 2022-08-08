

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

use super::searcher::{
    FileResult, 
    Match
};

// pub struct Location {
//     pub line_nr: usize,
//     pub file_path: String
// }


// pub struct MatchItem {
//     pub location: Location,
//     pub match_text: String
// }

pub struct App<'a> {
    results: Vec<FileResult>,
    list_state: ListState,
    curr_location: Location,
    list_items: Vec<ListItem<'a>>
}

struct Location {
    result_idx: usize,
    result_match_idx: usize
}



impl<'a> App<'a> {
    pub fn new(items : Vec<FileResult>) ->  App<'a> {
        let mut state = ListState::default();
        state.select(Some(1));
        let location = Location {result_idx: 0, result_match_idx: 0};

        let list_items: Vec<ListItem<'a>> = Self::make_list_items(&items);

        App{
            results: items,
            list_state: state,
            curr_location: location,
            list_items
        }
    }

    fn make_list_items(items: &Vec<FileResult>) -> Vec<ListItem<'a>>{
        let mut list_items: Vec<ListItem> = Vec::new();

        for res in items {
            list_items.push(
                ListItem::new(res.file_path.clone())
            );

            for m in &res.matches {
                list_items.push(ListItem::new(format!("{}: {}", m.line_nr.clone(), m.match_str.clone())));
            }
        }

        list_items
    }


    fn up(&mut self) {

        let curr_list_state = self.list_state.selected().unwrap();

        let new_state;
        if self.curr_location.result_match_idx == 0 {
            if self.curr_location.result_idx == 0 {
                return;
            }
            new_state = curr_list_state - 2;
            self.curr_location.result_idx -= 1;
            self.curr_location.result_match_idx = 
                self.results[self.curr_location.result_idx].matches.len() - 1;


        } else {
            new_state = curr_list_state - 1;
            self.curr_location.result_match_idx -= 1;
        }

        self.list_state.select(Some(
           new_state 
        ));

    }

    fn down(&mut self) {


        let curr = self.list_state.selected().unwrap();
        let res_match_idx = self.curr_location.result_idx;

        let new_selected;
        // at the last match of a file
        if self.curr_location.result_match_idx 
            == self.results[res_match_idx].matches.len() - 1 {
            if self.curr_location.result_idx == self.results.len() - 1 {
                // at the last match in the last file -- do nothing
                return;
            }

            new_selected = curr + 2;
            self.curr_location.result_idx += 1;
            self.curr_location.result_match_idx = 0;
        } else {
            self.curr_location.result_match_idx += 1;
            new_selected = curr + 1;
        }

        self.list_state.select(
            Some(
                new_selected
            )
        );
    }


    fn open_file(
        &mut self, 
    ) -> Result<(), Box<dyn Error>> {

        edit_file("hello.py").unwrap();
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

                                self.open_file().unwrap();

                                let mut stdout = io::stdout();
                                execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                                terminal.clear().unwrap();

                            },
                            KeyCode::Char('q') => {break;},
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
        term.draw(|f| {

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


            let file_list = List::new(self.list_items.clone())
                .block(fileblock)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            f.render_stateful_widget(file_list, filechunk, &mut self.list_state);


            let codeblock = Block::default()
                .title("Code")
                .borders(Borders::ALL);

            f.render_widget(codeblock, codechunk);
        })?;

        Ok(())
    }
}



   
