/*!
    This file contains the constants used in the project.
*/

use clap::builder::styling::{self, AnsiColor};

/// The styles for the CLI.
pub static CLI_STYLES: styling::Styles =
    styling::Styles::styled().usage(AnsiColor::BrightGreen.on_default());

/// The author of the project.
pub const AUTHOR: &str = "laplace";

/// The version of the project.
pub const VERSION_STR: &str = "\"v0.1.0\"";

/// The name of the project.
pub const PROJECT_NAME: &str = "search_code";

/// The about of the project. Actually, tell what this project does.
pub const ABOUT: &str = "use to search some code line in a file. 
Will highlight the key and give you the line number of the line.
";

/// The program language extensions.
pub const CPP_EXTENSIONS: [&str; 4] = ["cpp", "c", "cc", "cxx"];
pub const RUST_EXTENSIONS: [&str; 2] = ["rs", "rust"];
pub const PYTHON_EXTENSIONS: [&str; 1] = ["py"];

/// The query to search for the key in the code.
pub const CPP_MATCHES_QUERY: [&str; 3] = [
    r#"
((function_definition
  declarator:(
        function_declarator
            declarator:(identifier) @function
    )
)
            (#match? @function ":?"))
"#,
    r#"
((struct_specifier
    name:(type_identifier) @struct)
    (#match? @struct ":?"))
"#,
    r#"
((class_specifier
    name:(type_identifier) @class)
    (#match? @class ":?"))
"#,
];

pub const PYTHON_MATCHES_QUERY: [&str; 2] = [
    r#"
((function_definition
  name:(identifier) @function)
            (#match? @function ":?"))
"#,
    r#"
((class_definition
    name:(identifier) @class)
    (#match? @class ":?"))
"#,
];

pub const RUST_MATCHES_QUERY: [&str; 4] = [
    r#"
((function_item
  name:(identifier) @function)
            (#match? @function ":?"))
"#,
    r#"
((struct_item
    name:(type_identifier) @struct)
    (#match? @struct ":?"))
"#,
    r#"
((enum_item
    name:(type_identifier) @struct)
    (#match? @struct ":?"))
"#,
    r#"
((identifier) @constant
 (#match? @constant ":?"))
"#,
];
