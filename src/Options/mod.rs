/*! This module contains the submodules for the project.

    The submodules are:
    - cmd_args: This module contains the command line arguments parsing logic.
    - constants: This module contains the constants used in the project.
*/

pub mod cmd_args;
pub mod constants;

use tree_sitter::Language;
use tree_sitter_cpp;
use tree_sitter_python;
use tree_sitter_rust;

/** Lang enum to define for which language the search will be done.

    The possible values are:
    - C: will treat like Cpp
    - Cpp
    - Python
    - Rust
    - Any: This is the default value. just treat the file as raw text.

    *Use:*

    ```rust
    use search_code::Options::Lang;
    use tree_sitter::Query;

    let lang = Lang::from_str("cpp").unwrap();
    assert_eq!(lang, Lang::Cpp);
    ```
    *NOTE:* `search_query` is the query to search for the key in the code.
    This query is defined in the `constants.rs` file.
*/
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Lang {
    C,
    Cpp,
    Python,

    Rust,
    #[default]
    Any,
}

impl std::str::FromStr for Lang {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(obtain_lang(s))
    }
}

impl Lang {
    /** Convert the Lang to the tree-sitter language.

    *NOTE:* This function will return `Err(Lang::Any)` if the language is not supported.

    ```rust
    use search_code::Options::Lang;
    let tree_sitter_lang = parser.get_lang().into_treesitter_language().unwrap();
    Query::new(&tree_sitter_lang, search_query.replace(":?", key).as_str()).unwrap();
    ```
    */
    pub fn into_treesitter_language(&self) -> Result<Language, Lang> {
        match self {
            Lang::Cpp | Lang::C => Ok(tree_sitter_cpp::LANGUAGE.into()),
            Lang::Python => Ok(tree_sitter_python::LANGUAGE.into()),
            Lang::Rust => Ok(tree_sitter_rust::LANGUAGE.into()),
            _ => Err(Lang::Any),
        }
    }

    /// Get the valid extensions for the language.
    ///
    /// *Return:* The vector of the valid extensions. define in the `constants.rs` file.
    pub fn valid_extensions(&self) -> Vec<&str> {
        match self {
            Lang::Cpp | Lang::C => constants::CPP_EXTENSIONS.to_vec(),
            Lang::Python => constants::PYTHON_EXTENSIONS.to_vec(),
            Lang::Rust => constants::RUST_EXTENSIONS.to_vec(),
            _ => vec![],
        }
    }
}

fn obtain_lang(lang: &str) -> Lang {
    match lang {
        "c" | "cpp" | "cc" | "cxx" => Lang::Cpp,
        "python" | "Python" | "py" => Lang::Python,
        "rust" | "rs" => Lang::Rust,
        _ => Lang::default(),
    }
}

/** Get the query for the language.

*NOTE:* This function will return an empty vector if the language is not supported.

*Return:* The vector of the specify language queries. define in the `constants.rs` file.
*/
pub fn get_query(lang: &Lang) -> Vec<&'static str> {
    match lang {
        Lang::Cpp | Lang::C => constants::CPP_MATCHES_QUERY.to_vec(),
        Lang::Python => constants::PYTHON_MATCHES_QUERY.to_vec(),
        Lang::Rust => constants::RUST_MATCHES_QUERY.to_vec(),
        _ => vec![],
    }
}
