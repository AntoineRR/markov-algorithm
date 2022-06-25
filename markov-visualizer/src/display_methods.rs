use anyhow::Result;
use crossterm::{cursor, style, terminal, QueueableCommand};
use markov_runner::{nodes::Node, run_markov_1d, step_markov_1d};
use std::{
    io::{Stdout, Write},
    thread,
    time::Duration,
};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DisplayMethod {
    Evolutive,
    AllSteps,
    FinalResult,
}

pub fn evolutive(
    stdout: &mut Stdout,
    input: &str,
    node: &(impl Node + ?Sized),
    delay: Duration,
) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        stdout
            .queue(cursor::SavePosition)?
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))?
            .queue(cursor::RestorePosition)?
            .queue(style::Print(&r))?
            .queue(cursor::RestorePosition)?
            .flush()?;
        thread::sleep(delay);
        evolutive(stdout, &r, node, delay)?;
    }
    Ok(())
}

pub fn all_steps(stdout: &mut Stdout, input: &str, node: &(impl Node + ?Sized)) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        stdout.queue(style::Print(r.clone() + "\n"))?.flush()?;
        all_steps(stdout, &r, node)?;
    }
    Ok(())
}

pub fn final_result(stdout: &mut Stdout, input: &str, node: &(impl Node + ?Sized)) -> Result<()> {
    let result = run_markov_1d(input, node)?;
    stdout.queue(style::Print(result + "\n"))?.flush()?;
    Ok(())
}
