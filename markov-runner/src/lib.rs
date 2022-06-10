use std::error::Error;

pub struct Rule {
    pub input: String,
    pub output: String,
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
    return None;
}

pub fn run_markov_1d(input: &str, rules: &[Rule]) -> Result<String, Box<dyn Error>> {
    if let Some(r) = step_markov_1d(input, rules) {
        return run_markov_1d(&r, rules);
    } else {
        return Ok(input.to_owned());
    }
}

#[cfg(test)]
mod test {
    use crate::{run_markov_1d, Rule};

    #[test]
    fn basis() {
        let result = run_markov_1d(
            "AB",
            &[Rule {
                input: "AB".to_owned(),
                output: "BA".to_owned(),
            }],
        )
        .unwrap();
        assert_eq!(&result, "BA")
    }

    #[test]
    fn binary_converter() {
        let result = run_markov_1d(
            "110",
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
        )
        .unwrap();
        assert_eq!(&result, "xxxxxx")
    }
}
