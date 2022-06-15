use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{cursor, style, terminal, QueueableCommand};
use markov_runner::{
    nodes::{Node, Rule, Sequence},
    step_markov_1d,
};

use clap::Parser;

/// A simple visualization tool for Markov algorithms
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The algorithm to run
    #[clap(short, long, value_parser)]
    algorithm: Algorithm,

    /// The input for the algorithm
    #[clap(short, long, value_parser)]
    input: String,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Algorithm {
    BinaryToUnary,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut stdout = stdout();
    let node = match args.algorithm {
        Algorithm::BinaryToUnary => binary_to_unary_node(),
    };
    print_steps(&mut stdout, &args.input, &node, Duration::from_millis(500))?;
    Ok(())
}

fn binary_to_unary_node() -> Sequence {
    Sequence::new()
        .add_node(Rule::new("1", "0x"))
        .add_node(Rule::new("x0", "0xx"))
        .add_node(Rule::new("0", ""))
}

fn print_step(stdout: &mut Stdout, r: &str) -> Result<()> {
    stdout
        .queue(cursor::SavePosition)?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(cursor::RestorePosition)?
        .queue(style::Print(r))?
        .queue(cursor::RestorePosition)?
        .flush()?;
    Ok(())
}

fn print_steps<T: Node>(stdout: &mut Stdout, input: &str, node: &T, delay: Duration) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        print_step(stdout, &r)?;
        thread::sleep(delay);
        print_steps(stdout, &r, node, delay)?;
    }
    Ok(())
}
