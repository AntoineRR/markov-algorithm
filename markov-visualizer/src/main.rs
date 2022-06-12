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

fn main() -> Result<()> {
    let mut stdout = stdout();
    let node = Sequence::new()
        .add_node(Rule::new("1", "0x"))
        .add_node(Rule::new("x0", "0xx"))
        .add_node(Rule::new("0", ""));
    print_steps(&mut stdout, "101", &node, Duration::from_millis(500))?;
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

fn print_steps<T: Node>(stdout: &mut Stdout, input: &str, node: &T, delay: Duration) -> Result<()> {
    if let Some(r) = step_markov_1d(input, node) {
        print_step(stdout, &r)?;
        thread::sleep(delay);
        print_steps(stdout, &r, node, delay)?;
    }
    Ok(())
}
