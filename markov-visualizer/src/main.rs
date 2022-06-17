use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{cursor, style, terminal, QueueableCommand};
use markov_runner::{
    nodes::{Node, RandomChoice, Rule, Sequence},
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
    RandomMarch,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut stdout = stdout();
    let node = match args.algorithm {
        Algorithm::BinaryToUnary => binary_to_unary_node(),
        Algorithm::RandomMarch => random_march(),
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

fn random_march() -> Sequence {
    Sequence::new().add_node(Box::new(
        RandomChoice::new()
            .add_node(Rule::new("OXO", "OOX"))
            .add_node(Rule::new("OXO", "XOO")),
    ))
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

fn print_steps(stdout: &mut Stdout, input: &str, node: &impl Node, delay: Duration) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        print_step(stdout, &r)?;
        thread::sleep(delay);
        print_steps(stdout, &r, node, delay)?;
    }
    Ok(())
}
