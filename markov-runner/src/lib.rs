pub mod nodes;

use anyhow::Result;
use nodes::Node;

pub fn step_markov_1d<T: Node>(input: &str, node: &T) -> Option<String> {
    node.apply(input)
}

pub fn run_markov_1d<T: Node>(input: &str, node: &T) -> Result<String> {
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
