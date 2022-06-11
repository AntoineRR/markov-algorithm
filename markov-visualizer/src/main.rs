use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{cursor, style, terminal, QueueableCommand};
use markov_runner::{step_markov_1d, Rule};

fn main() -> Result<()> {
    let mut stdout = stdout();
    print_steps(
        &mut stdout,
        "101",
        &[
            Rule::new("1", "0x"),
            Rule::new("x0", "0xx"),
            Rule::new("0", ""),
        ],
        Duration::from_millis(500),
    )?;
    Ok(())
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

fn print_steps(stdout: &mut Stdout, input: &str, rules: &[Rule], delay: Duration) -> Result<()> {
    if let Some(r) = step_markov_1d(input, rules) {
        print_step(stdout, &r)?;
        thread::sleep(delay);
        print_steps(stdout, &r, rules, delay)?;
    }
    Ok(())
}
