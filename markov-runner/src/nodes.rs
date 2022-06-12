use std::fmt::Debug;

use rand::prelude::SliceRandom;

pub trait Node: Debug {
    fn apply(&self, input: &str) -> Option<String>;
}

#[derive(Debug, Default)]
pub struct Sequence {
    nodes: Vec<Box<dyn Node>>,
}

impl Sequence {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(mut self, node: Box<dyn Node>) -> Self {
        self.nodes.push(node);
        self
    }
}

impl Node for Sequence {
    fn apply(&self, input: &str) -> Option<String> {
        for node in &self.nodes {
            let result = node.apply(input);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct RandomChoice {
    nodes: Vec<Box<dyn Node>>,
}

impl RandomChoice {
    pub fn new() -> Self {
        RandomChoice { nodes: vec![] }
    }

    pub fn add_node(mut self, node: Box<dyn Node>) -> Self {
        self.nodes.push(node);
        self
    }
}

impl Node for RandomChoice {
    fn apply(&self, input: &str) -> Option<String> {
        let node = self.nodes.choose(&mut rand::thread_rng())?;
        node.apply(input)
    }
}

#[derive(Debug)]
pub struct Rule {
    pub input: String,
    pub output: String,
}

impl Rule {
    pub fn new(input: &str, output: &str) -> Box<Self> {
        Box::new(Self {
            input: input.to_owned(),
            output: output.to_owned(),
        })
    }
}

impl Node for Rule {
    fn apply(&self, input: &str) -> Option<String> {
        let mut result = input.to_owned();
        if let Some(index) = input.find(&self.input) {
            result.replace_range(index..index + self.input.len(), &self.output);
            return Some(result);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::nodes::{Node, RandomChoice, Rule, Sequence};

    #[test]
    fn rule_valid() {
        let rule = Rule::new("AB", "BA");
        assert_eq!(rule.apply("AB"), Some("BA".to_owned()));
    }

    #[test]
    fn rule_invalid() {
        let rule = Rule::new("AB", "BA");
        assert_eq!(rule.apply(".."), None);
    }

    #[test]
    fn sequence_valid() {
        let sequence = Sequence::new()
            .add_node(Rule::new("AB", "BA"))
            .add_node(Rule::new("BB", "AA"));
        assert_eq!(sequence.apply("ABB"), Some("BAB".to_owned()));

        let sequence = Sequence::new()
            .add_node(Rule::new("AB", "BA"))
            .add_node(Rule::new("BB", "AA"));
        assert_eq!(sequence.apply("BBA"), Some("AAA".to_owned()));
    }

    #[test]
    fn sequence_invalid() {
        let sequence = Sequence::new()
            .add_node(Rule::new("AB", "BA"))
            .add_node(Rule::new("BB", "AA"));
        assert_eq!(sequence.apply(".."), None);
    }

    #[test]
    fn random_choice_valid() {
        let rd = RandomChoice::new()
            .add_node(Rule::new("AB", "BA"))
            .add_node(Rule::new("BB", "AA"));
        assert!([Some("BAB".to_owned()), Some("AAA".to_owned())].contains(&rd.apply("ABB")));
    }

    #[test]
    fn random_choice_invalid() {
        let rd = Sequence::new()
            .add_node(Rule::new("AB", "BA"))
            .add_node(Rule::new("BB", "AA"));
        assert_eq!(rd.apply(".."), None);
    }
}
