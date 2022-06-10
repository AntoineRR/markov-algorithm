use std::error::Error;

pub struct Rule {
    pub input: String,
    pub output: String,
}

impl Rule {
    pub fn new(input: &str, output: &str) -> Self {
        Self {
            input: input.to_owned(),
            output: output.to_owned(),
        }
    }
}

fn get_random_match(input: &str, to_match: &str) -> Option<usize> {
    input.find(to_match)
}

pub fn step_markov_1d(input: &str, rules: &[Rule]) -> Option<String> {
    let mut result = input.to_owned();
    for rule in rules {
        if let Some(index) = get_random_match(&result, &rule.input) {
            result.replace_range(index..index + rule.input.len(), &rule.output);
            return Some(result);
        }
    }
    None
}

pub fn run_markov_1d(input: &str, rules: &[Rule]) -> Result<String, Box<dyn Error>> {
    if let Some(r) = step_markov_1d(input, rules) {
        run_markov_1d(&r, rules)
    } else {
        Ok(input.to_owned())
    }
}

#[cfg(test)]
mod test {
    use crate::{run_markov_1d, Rule};

    #[test]
    fn basis() {
        let result = run_markov_1d("AB", &[Rule::new("AB", "BA")]).unwrap();
        assert_eq!(&result, "BA")
    }

    #[test]
    fn binary_converter() {
        let result = run_markov_1d(
            "110",
            &[
                Rule::new("1", "0x"),
                Rule::new("x0", "0xx"),
                Rule::new("0", ""),
            ],
        )
        .unwrap();
        assert_eq!(&result, "xxxxxx")
    }
}
