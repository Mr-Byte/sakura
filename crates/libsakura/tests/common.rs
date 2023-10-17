use expect_test::expect_file;
use libsakura::{
    lexer::LexedStr,
    parser::{EntryPoint, LexedStrStep},
};
use std::{
    fs,
    io::Write,
    panic,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestCase {
    source: PathBuf,
    syntax: PathBuf,
}

impl TestCase {
    pub fn list(path: impl AsRef<Path>) -> Vec<TestCase> {
        let crate_root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_data_dir = crate_root_dir.join("data");
        let dir = test_data_dir.join(path);
        let read_dir = fs::read_dir(&dir)
            .unwrap_or_else(|err| panic!("unable to read directory {}: {err}", dir.display()));
        let mut listing = read_dir
            .filter_map(|file| {
                let file = file.unwrap();
                let path = file.path();

                if path.extension().unwrap_or_default() != "sk" {
                    return None;
                }

                let source = path.clone();
                let syntax = path.with_extension("skast");

                Some(TestCase { source, syntax })
            })
            .collect::<Vec<_>>();

        listing.sort();
        listing
    }

    pub fn run(self) -> bool {
        let test_name = self.source.file_stem().and_then(|s| s.to_str());
        let Some(test_name) = test_name else {
            // TODO: Is this a failure?
            return true;
        };

        let mut stdout = std::io::stdout().lock();
        let _ = write!(stdout, "test {test_name} ... ");

        let result = panic_control::spawn_quiet(move || {
            let source = fs::read_to_string(&self.source).expect("unable to read source file");
            let (result, _has_errors) = parse(EntryPoint::SourceFile, &source);
            let expected = expect_file!(&self.syntax);

            expected.assert_eq(&result);
        })
        .join();

        let result = match result {
            Ok(()) => {
                let _ = writeln!(stdout, "\x1b[32mok\x1b[0m");
                true
            }
            Err(err) => {
                let _ = writeln!(stdout, "\x1b[31mfailed\x1b[0m");

                if let Some(err) = err
                    .downcast_ref::<&str>()
                    .map(|err| *err)
                    .or_else(|| err.downcast_ref::<String>().map(|err| &**err))
                {
                    println!("reason: {}", err);
                }

                false
            }
        };

        drop(stdout);

        result
    }
}

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
