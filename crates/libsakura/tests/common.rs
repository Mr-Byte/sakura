use libsakura::{
    lexer::LexedStr,
    parser::{EntryPoint, LexedStrStep},
};

pub fn parse(entry: EntryPoint, input: &str) -> (String, bool) {
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
