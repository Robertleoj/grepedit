
use std::io::{self, Read};
use std::fs;


use grep::searcher::{
    Searcher,
    Sink,
    SinkMatch
};
use grep::regex::RegexMatcher;


pub fn search_file(
    file_path: &str, 
    pattern: &str, 
    // searcher: &mut Searcher
) -> Option<FileResult>{

    let r = RegexMatcher::new(pattern).unwrap();
    let mut searcher = Searcher::new();
    let mut result_vec = vec![];
    let m = MatchSink{matches: &mut result_vec};

    let f = fs::File::open(file_path).unwrap();

    searcher.search_file(r, &f, m).unwrap();

    if result_vec.len() > 0 {
        Some(FileResult{
            file_path: file_path.to_string(),
            matches : result_vec
        })
    }  else {
        None
    }
}

pub struct FileResult {
    pub file_path: String,
    pub matches: Vec<Match>
}

#[derive(Debug)]
pub struct Match {
    pub match_str: String,
    pub line_nr: u64
}

#[derive(Debug)]
struct MatchSink<'a> {
    matches: &'a mut Vec<Match>
}

impl<'a> Sink for MatchSink<'a> {
    type Error = io::Error;

    fn matched(&mut self, _searcher: &Searcher, _mat: &SinkMatch) -> Result<bool, std::io::Error> {

        let mut line_buff = _mat.lines().next().unwrap();
        let mut line = String::new();
        line_buff.read_to_string(&mut line).unwrap();

        self.matches.push(Match { match_str: line, line_nr: _mat.line_number().unwrap()});

        Ok(true)
    }
}



