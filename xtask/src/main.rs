mod codegen;
mod workspace;

use clap::{Parser, Subcommand};
use miette::Result;

fn main() -> Result<()> {
    let app = Xtask::parse();
    app.run()
}

#[derive(Debug, Parser)]
#[clap(name = "xtask")]
struct Xtask {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Perform code generation for the parser
    Codegen(codegen::Codegen),
}

impl Xtask {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Command::Codegen(command) => command.run(),
        }?;

        Ok(())
    }
}
