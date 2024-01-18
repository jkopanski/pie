extern crate pie;
use clap::Parser;
use miette::{IntoDiagnostic, Report, Result};
use pie::parser;
use std::{borrow::Cow, fs, path::PathBuf, println};

#[derive(Parser)]
#[command(name = "pie lang")]
#[command(author = "Jakub Kopański <jakub@famisoft.pl>")]
#[command(version = "0.0.1")]
#[command(about = "The Little Typer")]
#[command(long_about = "Learning dependent types by building a language")]
struct Opts {
    /// FILE to typecheck
    // #[arg(last = true)]
    file: PathBuf,
    /// TODO: look for imports in DIR
    #[arg(short, long, value_name = "DIR")]
    include: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let mut source = Cow::from(fs::read_to_string(opts.file).into_diagnostic()?);

    let prog =
        parser::parse(&mut source).map_err(|err| Report::new(err).with_source_code(source))?;

    for stmt in prog.statements {
        println!("{stmt:#?}");
    }
    Ok(())
}
