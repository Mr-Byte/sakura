use quote::{format_ident, quote};
use sourcegen::{
    format_output,
    syntax::{KindData, KIND_DATA},
};
use std::fs::File;
use std::io::Write as _;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let syntax_kinds = generate_syntax_kinds(KIND_DATA);
    let syntax_kinds_path = sourcegen::parser_src_dir().join("syntax_kind/generated.rs");
    let mut syntax_kinds_file = File::create(syntax_kinds_path)?;

    write!(syntax_kinds_file, "{syntax_kinds}")?;

    Ok(())
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
