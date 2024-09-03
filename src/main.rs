#![allow(non_snake_case)]

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use search_code::{
    parser_lang::SpecifyParser,
    Options::{
        cmd_args::{get_args_matches, CommandArgs},
        get_query, Lang,
    },
};

use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};
use std::{io, time::Duration};
use tree_sitter::{Query, QueryCursor};

fn main() {
    let matches = get_args_matches();
    let args = CommandArgs::from_matches(&matches);
    let mut search = SearchCode::build(&args.lang);
    if Path::new(args.path).is_dir() {
        if args.is_symbol {
            print_result(
                search
                    .walk_dir(Path::new(args.path), args.key_to_search, &get_symbols)
                    .expect(
                        "Can't search the key in the file. Maybe the file contains invalid utf-8 characters.",
                    ),
                args.key_to_search,
                colored::Color::Green,
            );
        } else {
            print_result(
                search
                    .walk_dir(Path::new(args.path), args.key_to_search, &find_key_file)
                    .expect(
                        "Can't search the key in the file. Maybe the file contains invalid utf-8 characters.",
                    ),
                args.key_to_search,
                colored::Color::Green,
            );
        }
    } else if args.is_symbol {
        print_result(
            get_symbols(args.path, args.key_to_search, &mut search.parser).unwrap(),
            args.key_to_search,
            colored::Color::Green,
        );
    } else {
        print_result(
            find_key_file(args.path, args.key_to_search, &mut search.parser).unwrap(),
            args.key_to_search,
            colored::Color::Green,
        );
    }
}

fn print_result(result: Vec<(String, usize, String)>, key: &str, color_paint: colored::Color) {
    for (filename, line_number, line) in result {
        let new_line = line.replace(key, key.color(color_paint).to_string().as_str());
        println!("in {}:{} => \t{}", filename, line_number, new_line);
    }
}

struct SearchCode {
    parser: SpecifyParser,
    lang: Lang,
    pathes: Vec<String>,
}

impl SearchCode {
    fn build(lang: &Lang) -> Self {
        Self {
            parser: SpecifyParser::from_lang(lang),
            lang: lang.clone(),
            pathes: vec![],
        }
    }

    /// Walk through the directory and do the operation.
    ///
    /// *Return:* : The vector of the found that contains key.
    /// - The first element is the filename.
    /// - The second element is the line number.
    /// - The third element is the line contains key.
    fn walk_dir<Operating>(
        &mut self,
        dirname: &Path,
        key: &str,
        op: &Operating,
    ) -> Result<Vec<(String, usize, String)>, io::Error>
    where
        Operating:
            Fn(&str, &str, &mut SpecifyParser) -> Result<Vec<(String, usize, String)>, io::Error>,
    {
        let mut found = vec![];
        self.scan_dir(dirname);
        let progress_bar = ProgressBar::new(self.pathes.len() as u64);
        progress_bar.set_style(
            ProgressStyle::with_template("[{elapsed}] {bar:40.cyan/blue} [{pos:>5}/{len:5}] {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        progress_bar.set_message(format!("Searching \"{key}\" ...").bright_blue().to_string());
        progress_bar.enable_steady_tick(Duration::from_millis(100));

        for path in &self.pathes {
            progress_bar.inc(1);
            let extension = path.split('.').last().unwrap();
            if self.lang != Lang::Any && !self.lang.valid_extensions().contains(&extension) {
                continue;
            }
            let result = op(path, key, &mut self.parser)?;
            found.extend(result);
        }
        progress_bar.finish();

        Ok(found)
    }

    /// Scan the directory and return the vector of the files' path.
    ///
    /// **Return:** The vector of the files' path.
    fn scan_dir(&mut self, dirname: &Path) {
        if dirname.is_dir() {
            for entry in std::fs::read_dir(dirname)
                .unwrap_or_else(|_| panic!("Can't read the directory: {:?}", dirname))
            {
                let entry = entry.expect("Can't read the entry");
                let path = entry.path();
                if path.is_dir() {
                    if path.file_name().unwrap() != ".git" {
                        self.scan_dir(&path)
                    };
                } else {
                    self.pathes.push(path.to_str().unwrap().to_string());
                }
            }
        } else {
            self.pathes.push(dirname.to_str().unwrap().to_string());
        }
    }
}

/// This is used when just search not support language.
///
/// **Supported languages are:**
/// - Cpp
/// - Python
/// - Rust
/// - C: treat as Cpp
///
fn find_key_file(
    filename: &str,
    key: &str,
    _placeholder: &mut SpecifyParser,
) -> Result<Vec<(String, usize, String)>, io::Error> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut found = vec![];
    for (line_number, line_str) in reader.lines().enumerate() {
        let line = line_str?;
        if line.contains(key) {
            found.push((filename.to_owned(), line_number + 1, line));
        }
    }

    Ok(found)
}

/** Get the symbols from the code.

    *NOTE:* This function calls only if lang is not `Any`.
    or panic;

    *Return:* The vector of the symbols.
    - The first element is the filename.
    - The second element is the line number.
    - The third element is the line contains symbol.
*/
pub fn get_symbols(
    filename: &str,
    key: &str,
    parser: &mut SpecifyParser,
) -> Result<Vec<(String, usize, String)>, io::Error> {
    let code = std::fs::read_to_string(filename).unwrap();
    let tree_sitter_lang = parser
        .get_lang()
        .into_treesitter_language()
        .expect(
"`Can't convert the language to the tree-sitter language. Maybe the language is not supported.
If you want to search with symbol, please use -l to specify the language.`"
        );
    let ast = parser.get_ast(&code);

    let mut query_cursor = QueryCursor::new();
    let mut symbols_vec = vec![];

    for search_query in get_query(parser.get_lang()) {
        let query =
            Query::new(&tree_sitter_lang, search_query.replace(":?", key).as_str()).unwrap();
        let captures = query_cursor.captures(&query, ast.root_node(), code.as_bytes());
        for (cs, cs_index) in captures {
            let capture = cs.captures[cs_index];
            let node = capture.node;
            let text = node.utf8_text(code.as_bytes()).expect(
                "`Can't get the text from the node. Maybe containes invalid utf-8 characters.`",
            );
            symbols_vec.push((
                filename.to_string(),
                node.start_position().row + 1,
                text.to_string(),
            ));
        }
    }

    Ok(symbols_vec)
}
