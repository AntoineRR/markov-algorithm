//! This crate provides a way to run Markov algorithms.

pub mod nodes;

use anyhow::Result;
use nodes::Node;

/// Run one step of the specified Markov algorithm.
///
/// # Example
/// ```
/// use markov_runner::nodes::Rule;
/// use markov_runner::step_markov_1d;
///
/// let node = Rule::new("AB", "BA");
/// assert!(step_markov_1d("ABAB", &node) == Some("BAAB".to_owned()));
/// ```
pub fn step_markov_1d(input: &str, node: &(impl Node + ?Sized)) -> Option<String> {
    node.apply(input)
}

/// Run the specified Markov algorithm to completion
///
/// # Example
/// ```
/// use markov_runner::nodes::Rule;
/// use markov_runner::run_markov_1d;
///
/// let node = Rule::new("AB", "BA");
/// assert!(run_markov_1d("ABAB", &node).unwrap() == "BBAA".to_owned());
/// ```
pub fn run_markov_1d(input: &str, node: &(impl Node + ?Sized)) -> Result<String> {
    if let Some(r) = step_markov_1d(input, node) {
        run_markov_1d(&r, node)
    } else {
        Ok(input.to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        nodes::{Rule, Sequence},
        run_markov_1d,
    };

    #[test]
    fn basis() {
        let node = Sequence::new().add_node(Rule::boxed("AB", "BA"));
        let result = run_markov_1d("AB", &node).unwrap();
        assert_eq!(&result, "BA")
    }

    #[test]
    fn binary_converter() {
        let node = Sequence::new()
            .add_node(Rule::boxed("1", "0x"))
            .add_node(Rule::boxed("x0", "0xx"))
            .add_node(Rule::boxed("0", ""));
        let result = run_markov_1d("110", &node).unwrap();
        assert_eq!(&result, "xxxxxx")
    }
}
