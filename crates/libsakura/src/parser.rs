///! Core parser for the Sakura Language, it returns [`ParserOutput`] which is a stream of [`ParserOutputStep`].
/// It is heavily based on and inspired by rust-analyzer.
mod event;
mod grammar;
mod input;
mod marker;
mod output;
mod parser;

pub(crate) mod tree_builder;
mod expressions;

pub use self::input::ParserInput;
pub use self::output::ParserOutput;
pub use self::output::ParserOutputStep;
pub use self::tree_builder::LexedStrStep;

pub enum EntryPoint {
    SourceFile,
}

impl EntryPoint {
    pub fn parse(&self, input: &ParserInput) -> ParserOutput {
        let mut parser = parser::Parser::new(input);

        self.entry_point()(&mut parser);

        let events = parser.finish();
        let result = event::process(events);

        if cfg!(debug_assertions) {
            let mut depth = 0;
            let mut first = true;

            for step in result.iter() {
                assert!(depth > 0 || first);
                first = false;

                match step {
                    ParserOutputStep::Enter { .. } => depth += 1,
                    ParserOutputStep::Exit => depth -= 1,
                    ParserOutputStep::Token { .. } | ParserOutputStep::Error { .. } => (),
                }
            }

            assert!(!first, "No tree parsed.");
            assert_eq!(depth, 0, "Unbalanced tree.");
        }

        result
    }

    fn entry_point(&self) -> fn(&'_ mut parser::Parser<'_>) {
        match self {
            EntryPoint::SourceFile => grammar::entry::top::source_file,
        }
    }
}
