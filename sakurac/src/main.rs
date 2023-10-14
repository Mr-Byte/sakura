use clap::{Parser, ValueEnum};
use libsakura::syntax::ast::{AstNode, Item};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<PathBuf>,

    #[clap(value_enum, default_value_t = Mode::PrintTree)]
    mode: Mode,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Mode {
    PrintTree,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input = args.file.map_or_else(
        || Box::new(std::io::stdin()) as Box<dyn std::io::Read>,
        |path| Box::new(std::fs::File::open(path).unwrap()) as Box<dyn std::io::Read>,
    );

    match args.mode {
        Mode::PrintTree => print_items(input)?,
    }

    Ok(())
}

fn print_items(mut reader: impl std::io::Read) -> anyhow::Result<()> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let result = libsakura::SourceFile::parse(&buffer);

    for item in result.tree().items() {
        match item {
            Item::TypeDeclaration(type_decl) => {
                dbg!(type_decl);
            }
            Item::FunctionDeclaration(_) => {}
        }
    }

    Ok(())
}

fn print_tree(mut reader: impl std::io::Read) -> anyhow::Result<()> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let result = libsakura::SourceFile::parse(&buffer);

    println!("Syntax Tree:\n");
    println!("{:#?}", result.syntax_node());
    println!("====================\n");
    println!("Errors:");

    for error in result.errors() {
        println!("{} @ {:#?}", error.message, error.text_range);
    }

    Ok(())
}
