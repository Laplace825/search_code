/*! the tree-sitter parser to parse the code.

    Now can only support the `c cpp python rust` language
*/

use tree_sitter::Parser;

/// Should use `&xxx.into()` to set_language
use tree_sitter_cpp::LANGUAGE as cpp;
use tree_sitter_python::LANGUAGE as python;
use tree_sitter_rust::LANGUAGE as rust;

use crate::Options::Lang;

/// The parser to parse the code.
///
/// *NOTE:* use tree-sitter to parse code.
#[derive(Default)]
pub struct SpecifyParser {
    lang: Lang,
    parser: Parser,
}

impl SpecifyParser {
    /// get a default SpecifyParser
    pub fn new() -> Self {
        Default::default()
    }

    /// get a SpecifyParser from a language.
    ///
    /// if the language is not supported, will return a default SpecifyParser.
    pub fn from_lang(lang: &Lang) -> SpecifyParser {
        let mut parser = Parser::new();
        let error_loading_msg = "Error loading the language";
        match lang {
            Lang::Cpp | Lang::C => parser.set_language(&cpp.into()).expect(error_loading_msg),
            Lang::Python => parser
                .set_language(&python.into())
                .expect(error_loading_msg),
            Lang::Rust => parser.set_language(&rust.into()).expect(error_loading_msg),
            _ => Default::default(),
        };

        SpecifyParser {
            lang: lang.clone(),
            parser,
        }
    }

    /// use tree-sitter parser to get the ast tree
    pub fn get_ast(&mut self, code: &str) -> tree_sitter::Tree {
        self.parser.parse(code, None).unwrap()
    }

    /// get the language of the parser
    ///
    /// *Return:* Type `enum Lang`.
    pub fn get_lang(&self) -> &Lang {
        &self.lang
    }
}
