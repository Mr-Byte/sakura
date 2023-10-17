use libsakura::lexer::LexedStr;
use libsakura::parser::{EntryPoint, LexedStrStep};
use std::fs;
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::Once;

static HANDLER: Once = Once::new();

fn parse(entry: EntryPoint, input: &str) -> (String, bool) {
    use std::fmt::Write;

    let lexed = LexedStr::new(input);
    let parser_input = lexed.as_input();
    let parser_output = entry.parse(&parser_input);

    let mut buffer = String::new();
    let mut errors = Vec::new();
    let mut depth = 0;
    let mut indent = String::new();
    let mut len = 0;

    lexed.with_output(&parser_output, |_event| match _event {
        LexedStrStep::Token { kind, text } => {
            assert!(depth > 0);
            len += text.len();

            writeln!(buffer, "{indent}{kind:?} {text:?}").unwrap();
        }
        LexedStrStep::Enter { kind } => {
            assert!(depth > 0 || len == 0);
            writeln!(buffer, "{indent}{kind:?}").unwrap();

            depth += 1;
            indent.push_str("  ");
        }
        LexedStrStep::Exit => {
            assert!(depth > 0);
            depth -= 1;
            indent.pop();
            indent.pop();
        }
        LexedStrStep::Error { message, position } => {
            assert!(depth > 0);

            errors.push(format!("error {position}: {message}\n"));
        }
    });

    assert_eq!(
        len,
        input.len(),
        "Didn't parse all input.\nParsed:\n{}\n\nAll:\n{}\n",
        &input[..len],
        input
    );

    for (position, message) in lexed.errors() {
        let pos = lexed.text_range(position).start;
        errors.push(format!("error {pos}: {message}\n"));
    }

    let has_errors = !errors.is_empty();
    for err in errors {
        buffer.push_str(&err);
    }

    (buffer, has_errors)
}

pub struct TestCase {
    source: PathBuf,
    syntax: PathBuf,
}

impl TestCase {
    pub fn list(path: impl AsRef<Path>) -> impl Iterator<Item = TestCase> {
        let crate_root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_data_dir = crate_root_dir.join("data");
        let dir = test_data_dir.join(path);

        let read_dir = fs::read_dir(&dir)
            .unwrap_or_else(|err| panic!("unable to read directory {}: {err}", dir.display()));

        read_dir.filter_map(|file| {
            let file = file.unwrap();
            let path = file.path();

            if path.extension().unwrap_or_default() != "sk" {
                return None;
            }

            let source = path.clone();
            let syntax = path.with_extension("skast");

            Some(TestCase { source, syntax })
        })
    }

    pub fn run(self) -> bool {
        install_handler();

        let test_name = self.source.file_stem().and_then(|s| s.to_str());
        let Some(test_name) = test_name else {
            // TODO: Is this a failure?
            return true;
        };

        print!("test {test_name} ... ");

        let result = panic::catch_unwind(move || {
            let source = fs::read_to_string(&self.source).expect("unable to read source file");
            let syntax = fs::read_to_string(&self.syntax).expect("unable to read syntax file");

            let (result, _has_errors) = parse(EntryPoint::SourceFile, &source);

            assert_eq!(result, syntax, "syntax tree mismatch");
        });

        match result {
            Ok(()) => {
                println!("\x1b[32mok\x1b[0m");
                true
            }
            Err(err) => {
                println!("\x1b[31mfailed\x1b[0m");

                if let Some(err) = err
                    .downcast_ref::<&str>()
                    .map(|err| *err)
                    .or_else(|| err.downcast_ref::<String>().map(|err| &**err))
                {
                    println!("reason: {}", err);
                }

                false
            }
        }
    }
}

pub fn install_handler() {
    HANDLER.call_once(|| {
        let hook = panic::take_hook();

        panic::set_hook(Box::new(move |info| {
            if info
                .location()
                .map_or(false, |location| !location.file().ends_with("tests/common.rs"))
            {
                hook(info);
            }
        }));
    })
}
