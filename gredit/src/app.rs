

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
    pub items: Vec<MatchItem>,
    pub list_state: ListState,
}


impl App {
    pub fn up(&mut self) {
        self.list_state.select(
            Some(
                match self.list_state.selected().unwrap() {
                    0 => 0,
                    a => a - 1,
                }
            )
        );
    }

    pub fn down(&mut self) {
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

    fn render_plain_block<B: Backend>(&mut self, term: &mut Terminal<B>) ->  Result<(), Box<dyn Error>> {
        term.draw(|mut f| {
            f.render_widget(Block::default(), f.size());
        });

        Ok(())
    }

    pub fn open_file<B: Backend>(
        &mut self, 
        term: &mut Terminal<B>
    ) -> Result<(), Box<dyn Error>> {

        edit_file("hello.py");

        // self.render_plain_block(term);

        Ok(())

    }

    pub fn draw<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
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



   
