use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{cursor, style, terminal, QueueableCommand};
use markov_runner::{
    step_markov_1d,
    tokens::{Rule, Sequence, Token},
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    let token = Sequence::new()
        .add_rule(Rule::new("1", "0x"))
        .add_rule(Rule::new("x0", "0xx"))
        .add_rule(Rule::new("0", ""));
    print_steps(&mut stdout, "101", &token, Duration::from_millis(500))?;
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

fn print_steps<T: Token>(
    stdout: &mut Stdout,
    input: &str,
    token: &T,
    delay: Duration,
) -> Result<()> {
    if let Some(r) = step_markov_1d(input, token) {
        print_step(stdout, &r)?;
        thread::sleep(delay);
        print_steps(stdout, &r, token, delay)?;
    }
    Ok(())
}
