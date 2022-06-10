use std::{thread, time::Duration};

use markov_runner::{step_markov_1d, Rule};

fn main() {
    print_steps(
        "1101",
        &[
            Rule {
                input: "1".to_owned(),
                output: "0x".to_owned(),
            },
            Rule {
                input: "x0".to_owned(),
                output: "0xx".to_owned(),
            },
            Rule {
                input: "0".to_owned(),
                output: "".to_owned(),
            },
        ],
        Duration::from_millis(500),
    )
}

fn clear_line() {
    print!("\x1B[2J");
}

fn print_steps(input: &str, rules: &[Rule], delay: Duration) {
    if let Some(r) = step_markov_1d(input, rules) {
        clear_line();
        println!("{r}");
        thread::sleep(delay);
        print_steps(&r, rules, delay);
    }
}
