

use tui::{
    backend::{CrosstermBackend, Backend},
    Terminal,
    widgets::{Widget, Block, Borders, List, ListItem, ListState, Paragraph},
    layout::{Layout, Constraint, Direction},
    Frame,
    style::{Color, Style, Modifier},
    text::{Text, Spans, Span}
};

use std::{io, fs::File};
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

use syntect::parsing::SyntaxSet;
// use syntect::highlighting::{ThemeSet, Style};
use syntect::highlighting as highl;

use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightFile;
use std::io::BufRead;

type Res<T>= Result<T, Box<dyn Error>>;

fn highlight_file(file_path: &str) -> Res<Vec<Vec<(highl::Style, String)>>> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = highl::ThemeSet::load_defaults();

    let mut highlighter = HighlightFile::new(file_path, &ss, &ts.themes["base16-ocean.dark"]).unwrap();
    let mut line = String::new();

    let mut out: Vec<Vec<(highl::Style, String)>> = Vec::new();
    while highlighter.reader.read_line(&mut line)? > 0 {
        {
            let regions: Vec<(highl::Style, &str)> = highlighter.highlight_lines.highlight_line(&line, &ss).unwrap();
            // out.push_str(as_24_bit_terminal_escaped(&regions[..], false).as_str());
            let span_vec = regions.into_iter().map(|(hl, s)|{
                (hl, s.to_string())
            });
            out.push(span_vec.collect());

            // print!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        } // until NLL this scope is needed so we can clear the buffer after
        line.clear(); // read_line appends so we need to clear between lines
    }

    Ok(out)

}


pub struct App<'a> {
    results: Vec<FileResult>,
    list_state: ListState,
    curr_location: ResultListLocation,
    list_items: Vec<ListItem<'a>>,
    curr_file: Vec<Vec<(highl::Style, String)>>
}

struct ResultListLocation {
    result_idx: usize,
    result_match_idx: usize
}

struct FileLocation<'a> {
    file_path: &'a str,
    line_nr: u64
}


impl<'a> App<'a> {
    pub fn new(items : Vec<FileResult>) ->  App<'a> {
        let mut state = ListState::default();
        state.select(Some(1));
        let location = ResultListLocation {result_idx: 0, result_match_idx: 0};

        let list_items: Vec<ListItem<'a>> = Self::make_list_items(&items);

        let fpath = &items.get(0).as_ref().unwrap().file_path;
        let highlighted = highlight_file(fpath.as_str()).unwrap();

        App{
            results: items,
            list_state: state,
            curr_location: location,
            list_items,
            curr_file: highlighted
        }
    }

    fn make_styled(&self) -> Vec<Spans> {

        let mut  out = vec![];

        for (idx, line) in self.curr_file.iter().enumerate() {
            let spans: Vec<_> = line.iter().map(|(hl, s)|{
                let fg = hl.foreground;
                let bg = hl.background;
                let mut style = Style::default()
                    .fg(
                        Color::Rgb(fg.r, fg.g, fg.b)
                    );

                if idx + 1 == self.line_nr() as usize{
                    let bg = Color::Rgb(23, 30, 102);
                    style = style.bg(
                        bg
                    );
                }
                Span::styled(s, style)
            }).collect();

            out.push(Spans::from(spans));
        }

        out
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

    fn result_idx(&self) -> usize{
        self.curr_location.result_idx
    }

    fn result_match_idx(&self) -> usize {
        self.curr_location.result_match_idx
    }

    fn curr_file_location(&self) -> FileLocation {
        let curr_result 
            = &self.results[self.result_idx()];
        let curr_match = &curr_result.matches[self.result_match_idx()];

        FileLocation { file_path: &curr_result.file_path, line_nr: curr_match.line_nr }
    }


    fn open_file(
        &mut self, 
    ) -> Result<(), Box<dyn Error>> {

        edit_file(self.curr_file_location().file_path).unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        Ok(())
    }

    fn line_nr(&self) -> u64 {
        self.curr_file_location().line_nr
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
                            KeyCode::Up | KeyCode::Char('k') => {
                                info!("Pressed UP");
                                self.up();
                            },
                            KeyCode::Down | KeyCode::Char('j') => {
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

            let text_space = codeblock.inner(codechunk);


            let line_nr = self.line_nr();

            let line_lower = line_nr.saturating_sub(10);

            let line_upper = std::cmp::min(line_nr + 20, self.curr_file.len() as u64);
            let p = Paragraph::new::<Vec<_>>(
                self.make_styled()[(line_lower as usize)..(line_upper as usize)]
                    .iter().map(|l|{
                        l.clone()
                    }).collect()
            ).block(
                codeblock
            );

            f.render_widget(p, codechunk);



        })?;

        Ok(())
    }
}



   
