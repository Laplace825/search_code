/*!
    The CLI arguments to the config.
*/
use std::str::FromStr;

use clap::{Arg, ArgMatches, Command};

use crate::Options::constants::*;
use crate::Options::Lang;

/** Get the command line arguments.

*NOTE:* This set the about, version, author and the basic command arguments.

*RETURNS:* The matches from the command line.

```rust
use search_code::Options::cmd_args::get_args_matches;

let matches = get_args_matches();
```

*/
pub fn get_args_matches() -> ArgMatches {
    Command::new(PROJECT_NAME)
        .long_about(ABOUT)
        .version(VERSION_STR)
        .author(AUTHOR)
        .args([
            Arg::new("path")
                .help("The path to the file to search in")
                .short('p')
                .default_value("."),
            Arg::new("key_to_search")
                .help("The key to search for in the file")
                .short('k')
                .required(true),
            Arg::new("lang")
                .help("The language of the file")
                .short('l')
                .default_value("any"),
            Arg::new("symbol")
                .help("The symbol to search for in the file")
                .short('s')
                .default_value("true"),
        ])
        .styles(CLI_STYLES.to_owned())
        .get_matches()
}

/** command line arguments to config.

*PARAM:* {path} The path to the file to search in.

*PARAM:* {key_to_search} The key to search for in the file.

*PARAM:* {lang} The language of the file.
*/
#[derive(Debug, Default)]
pub struct CommandArgs<'a> {
    pub path: &'a str,
    pub key_to_search: &'a str,
    pub lang: Lang,
    pub is_symbol: bool,
}

impl<'a> CommandArgs<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /** Create a new instance of CommandArgs from the matches.


    *NOTE:* this function won't be panic because the matches must have to be setted.

    *PARAM:* {matches} The matches from the command line.

    *Use:*

    ```rust
    use search_code::Options::cmd_args::CommandArgs;
    use search_code::Options::cmd_args::get_args_matches;

    let matches = get_args_matches();
    let args = CommandArgs::from_matches(&matches);
    ```

    */
    pub fn from_matches(matches: &'a ArgMatches) -> CommandArgs<'a> {
        let path = matches.get_one::<String>("path").unwrap();
        let key_to_search = matches.get_one::<String>("key_to_search").unwrap();
        let is_symbol = matches.get_one::<String>("symbol").unwrap();

        CommandArgs {
            path,
            key_to_search,
            lang: Lang::from_str(matches.get_one::<String>("lang").unwrap()).unwrap(),
            is_symbol: is_symbol == "true",
        }
    }
}
