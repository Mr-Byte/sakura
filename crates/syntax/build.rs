use quote::quote;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use quote::format_ident;
use ungrammar::Grammar;

const SRC_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
const GRAMMAR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sakura.ungram"));

type StrSlice = &'static [&'static str];

struct KindData {
    punctuation: &'static [(&'static str, &'static str)],
    keywords: StrSlice,
    tokens: StrSlice,
    nodes: StrSlice,
}

const KIND_DATA: KindData = KindData {
    punctuation: &[
        (";", "Semicolon"),
        (",", "Comma"),
        ("(", "LeftParen"),
        (")", "RightParen"),
        ("{", "LeftCurly"),
        ("}", "RightCurly"),
        ("[", "LeftBracket"),
        ("]", "RightBracket"),
        ("<", "LeftAngle"),
        (">", "RightAngle"),
        ("@", "At"),
        ("#", "Pound"),
        ("~", "Tilde"),
        ("?", "Question"),
        ("$", "Dollar"),
        ("&", "Amp"),
        ("|", "Pipe"),
        ("+", "Plus"),
        ("*", "Star"),
        ("/", "Slash"),
        ("^", "Caret"),
        ("%", "Percent"),
        ("_", "Underscore"),
        (".", "Dot"),
        ("..", "DoubleDot"),
        ("..=", "DoubleDotEq"),
        (":", "Colon"),
        ("::", "DoubleColon"),
        ("=", "Eq"),
        ("==", "DoubleEqual"),
        ("=>", "FatArrow"),
        ("!", "Bang"),
        ("!=", "NotEq"),
        ("-", "Minus"),
        ("->", "ThinArrow"),
        ("<=", "LtEq"),
        (">=", "GtEq"),
        ("+=", "PusEq"),
        ("-=", "MinusEq"),
        ("|=", "PipeEq"),
        ("&=", "AmpEq"),
        ("^=", "CaretEq"),
        ("/=", "SlashEq"),
        ("*=", "StarEq"),
        ("%=", "PercentEq"),
        ("&&", "DoubleAmp"),
        ("||", "DoublePipe"),
        ("<<", "ShiftLeft"),
        (">>", "ShiftRight"),
        ("<<=", "ShiftLeftEq"),
        (">>=", "ShiftRightEq"),
    ],
    keywords: &[],
    tokens: &["WhiteSpace", "Comment"],
    nodes: &["SourceFile"],
};

fn main() {
    let grammar: Grammar = GRAMMAR.parse().unwrap();

    let syntax_kinds = generate_syntax_kinds(KIND_DATA);
    let mut syntax_kinds_file =
        File::create(Path::new(SRC_ROOT).join("../../parser/src/syntax_kind/generated.rs"))
            .expect("");

    write!(syntax_kinds_file, "{syntax_kinds}").expect("");

    lower_grammar(&grammar);
}

fn generate_syntax_kinds(kind_data: KindData) -> String {
    let punctuation =
        kind_data.punctuation.iter().map(|(_, name)| format_ident!("{}", name)).collect::<Vec<_>>();

    let tokens = kind_data.tokens.iter().map(|name| format_ident!("{name}")).collect::<Vec<_>>();
    let nodes = kind_data.nodes.iter().map(|name| format_ident!("{name}")).collect::<Vec<_>>();

    let syntax_kinds = quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            EOF,
            #(#punctuation,)*
            #(#tokens,)*
            #(#nodes,)*
            // Used for safe cast from u16
            #[doc(hidden)]
            __LAST,
        }
    };

    format_output(syntax_kinds.to_string())
}

fn lower_grammar(grammar: &Grammar) {
    let nodes = grammar.iter().collect::<Vec<_>>();

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;

        println!("{name}");
        println!("{rule:?}");
    }
}

fn format_output(source: String) -> String {
    let shell = xshell::Shell::new().expect("unable to open shell");
    let mut stdout =
        xshell::cmd!(shell, "rustfmt").stdin(source).read().expect("failed to unwrap rustfmt");

    stdout.insert_str(0, "//! Generated code, do not edit by hand.\n\n");
    stdout.push('\n');

    stdout
}
