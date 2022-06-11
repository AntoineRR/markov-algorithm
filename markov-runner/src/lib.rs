pub mod tokens;

use anyhow::Result;
use tokens::Token;

pub fn step_markov_1d<T: Token>(input: &str, token: &T) -> Option<String> {
    token.apply(input)
}

pub fn run_markov_1d<T: Token>(input: &str, token: &T) -> Result<String> {
    if let Some(r) = step_markov_1d(input, token) {
        run_markov_1d(&r, token)
    } else {
        Ok(input.to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        run_markov_1d,
        tokens::{Rule, Sequence},
    };

    #[test]
    fn basis() {
        let token = Sequence::new().add_rule(Rule::new("AB", "BA"));
        let result = run_markov_1d("AB", &token).unwrap();
        assert_eq!(&result, "BA")
    }

    #[test]
    fn binary_converter() {
        let token = Sequence::new()
            .add_rule(Rule::new("1", "0x"))
            .add_rule(Rule::new("x0", "0xx"))
            .add_rule(Rule::new("0", ""));
        let result = run_markov_1d("110", &token).unwrap();
        assert_eq!(&result, "xxxxxx")
    }
}
