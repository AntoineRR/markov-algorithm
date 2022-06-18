mod algorithms;
mod display_methods;

use std::{io::stdout, time::Duration};

use algorithms::{binary_to_unary_node, random_march, Algorithm};
use anyhow::Result;
use display_methods::{all_steps, evolutive, final_result, DisplayMethod};

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
