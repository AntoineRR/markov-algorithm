//! The nodes you can use to write your Markov algorithms.

use std::fmt::Debug;

use rand::prelude::SliceRandom;

pub trait Node: Debug {
    fn apply(&self, input: &str) -> Option<String>;
}

/// Sequence allows to specify a list of rules to apply in a specific order.
/// If the first rule cannot be applied, the second rule will be tried, and so on.
///
/// # Example
/// ```
/// use markov_runner::nodes::{Node, Rule, Sequence};
///
/// let sequence = Sequence::new()
///     .add_node(Rule::boxed("AB", "BA"))
///     .add_node(Rule::boxed("BB", "AA"));
/// assert_eq!(sequence.apply("ABB"), Some("BAB".to_owned()));
/// assert_eq!(sequence.apply("BBA"), Some("AAA".to_owned()));
/// ```
#[derive(Debug, Default)]
pub struct Sequence {
    nodes: Vec<Box<dyn Node>>,
}

impl Sequence {
    /// Create a new Sequence.
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Add a new Node to the Sequence.
    /// This new Node will apply only if all the previous ones failed to apply.
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

/// RandomChoice allows to specify a set of rules from which a random one will be picked.
///
/// # Example
/// ```
/// use markov_runner::nodes::{Node, Rule, RandomChoice};
///
/// let rd = RandomChoice::new()
///     .add_node(Rule::boxed("AB", "BA"))
///     .add_node(Rule::boxed("BB", "AA"));
/// assert!([Some("BAB".to_owned()), Some("AAA".to_owned())].contains(&rd.apply("ABB")));
/// ```
#[derive(Debug, Default)]
pub struct RandomChoice {
    nodes: Vec<Box<dyn Node>>,
}

impl RandomChoice {
    /// Create a new RandomChoice.
    pub fn new() -> Self {
        RandomChoice { nodes: vec![] }
    }

    /// Add a Node that can randomly be chosen when applying the RandomChoice.
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

/// A rule seeks a specified pattern inside the input and turns it into the specified output.
///
/// # Example
/// ```
/// use markov_runner::nodes::{Node, Rule};
///
/// let rule = Rule::new("AB", "BA");
/// assert!(rule.apply("ABB") == Some("BAB".to_owned()));
/// ```
#[derive(Debug)]
pub struct Rule {
    pub pattern: String,
    pub output: String,
}

impl Rule {
    /// Create a new Rule.
    pub fn new(pattern: &str, output: &str) -> Self {
        Self {
            pattern: pattern.to_owned(),
            output: output.to_owned(),
        }
    }

    /// Create a new boxed Rule.
    /// This is useful for adding the rule to a Sequence for example.
    pub fn boxed(input: &str, output: &str) -> Box<Self> {
        Box::new(Self::new(input, output))
    }
}

impl Node for Rule {
    fn apply(&self, input: &str) -> Option<String> {
        let mut result = input.to_owned();
        // TODO: introduce a pattern matching strategy.
        if let Some(index) = input.find(&self.pattern) {
            result.replace_range(index..index + self.pattern.len(), &self.output);
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
            .add_node(Rule::boxed("AB", "BA"))
            .add_node(Rule::boxed("BB", "AA"));
        assert_eq!(sequence.apply("ABB"), Some("BAB".to_owned()));
        assert_eq!(sequence.apply("BBA"), Some("AAA".to_owned()));
    }

    #[test]
    fn sequence_invalid() {
        let sequence = Sequence::new()
            .add_node(Rule::boxed("AB", "BA"))
            .add_node(Rule::boxed("BB", "AA"));
        assert_eq!(sequence.apply(".."), None);
    }

    #[test]
    fn random_choice_valid() {
        let rd = RandomChoice::new()
            .add_node(Rule::boxed("AB", "BA"))
            .add_node(Rule::boxed("BB", "AA"));
        assert!([Some("BAB".to_owned()), Some("AAA".to_owned())].contains(&rd.apply("ABB")));
    }

    #[test]
    fn random_choice_invalid() {
        let rd = Sequence::new()
            .add_node(Rule::boxed("AB", "BA"))
            .add_node(Rule::boxed("BB", "AA"));
        assert_eq!(rd.apply(".."), None);
    }
}
