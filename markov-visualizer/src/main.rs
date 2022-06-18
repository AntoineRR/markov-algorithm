use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{cursor, style, terminal, QueueableCommand};
use markov_runner::{
    nodes::{Node, RandomChoice, Rule, Sequence},
    run_markov_1d, step_markov_1d,
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

    /// The method to use to display the result of the algorithm
    #[clap(short, long, value_parser)]
    display_method: Option<DisplayMethod>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum Algorithm {
    BinaryToUnary,
    RandomMarch,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum DisplayMethod {
    Evolutive,
    AllSteps,
    FinalResult,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut stdout = stdout();
    let node = match args.algorithm {
        Algorithm::BinaryToUnary => binary_to_unary_node(),
        Algorithm::RandomMarch => random_march(),
    };
    match args.display_method {
        Some(DisplayMethod::Evolutive) => {
            evolutive(&mut stdout, &args.input, &node, Duration::from_millis(500))
        }
        Some(DisplayMethod::AllSteps) => all_steps(&mut stdout, &args.input, &node),
        Some(DisplayMethod::FinalResult) => final_result(&mut stdout, &args.input, &node),
        None => all_steps(&mut stdout, &args.input, &node),
    }
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

fn clear_and_print_step(stdout: &mut Stdout, r: &str) -> Result<()> {
    stdout
        .queue(cursor::SavePosition)?
        .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
        .queue(cursor::RestorePosition)?
        .queue(style::Print(r))?
        .queue(cursor::RestorePosition)?
        .flush()?;
    Ok(())
}

fn evolutive(stdout: &mut Stdout, input: &str, node: &impl Node, delay: Duration) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        clear_and_print_step(stdout, &r)?;
        thread::sleep(delay);
        evolutive(stdout, &r, node, delay)?;
    }
    Ok(())
}

fn all_steps(stdout: &mut Stdout, input: &str, node: &impl Node) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        stdout.queue(style::Print(r.clone() + "\n"))?.flush()?;
        all_steps(stdout, &r, node)?;
    }
    Ok(())
}

fn final_result(stdout: &mut Stdout, input: &str, node: &impl Node) -> Result<()> {
    let result = run_markov_1d(input, node)?;
    stdout.queue(style::Print(result + "\n"))?.flush()?;
    Ok(())
}
