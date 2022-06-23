//! The nodes you can use to write your Markov algorithms.

use std::fmt::Debug;

use rand::prelude::{IteratorRandom, SliceRandom};

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

/// The matching strategy to adopt for a Rule.
/// - First : the first occurence of the pattern in the input will be chosen.
/// - Last : the last occurence of the pattern in the input will be chosen.
/// - Random : a random occurence of the pattern in the input will be chosen.
#[derive(Debug)]
pub enum MatchingStrategy {
    First,
    Last,
    Random,
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
    pub matching_strategy: MatchingStrategy,
}

impl Rule {
    /// Create a new Rule.
    pub fn new(pattern: &str, output: &str) -> Self {
        Self {
            pattern: pattern.to_owned(),
            output: output.to_owned(),
            matching_strategy: MatchingStrategy::First,
        }
    }

    /// Create a new boxed Rule.
    /// This is useful for adding the rule to a Sequence for example.
    pub fn boxed(input: &str, output: &str) -> Box<Self> {
        Box::new(Self::new(input, output))
    }

    /// Set the matching strategy for this Rule.
    pub fn set_matching_strategy(mut self, matching_strategy: MatchingStrategy) -> Self {
        self.matching_strategy = matching_strategy;
        self
    }
}

impl Node for Rule {
    fn apply(&self, input: &str) -> Option<String> {
        let mut result = input.to_owned();
        let index = match self.matching_strategy {
            MatchingStrategy::First => input.find(&self.pattern),
            MatchingStrategy::Last => input.rfind(&self.pattern),
            MatchingStrategy::Random => input
                .as_bytes()
                .windows(self.pattern.len())
                .zip(0..input.len())
                .filter(|x| x.0 == self.pattern.as_bytes())
                .map(|x| x.1)
                .choose(&mut rand::thread_rng()),
        };
        if let Some(index) = index {
            result.replace_range(index..index + self.pattern.len(), &self.output);
            return Some(result);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::nodes::{MatchingStrategy, Node, RandomChoice, Rule, Sequence};

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
    fn rule_matching_strategy() {
        let rule = Rule::new("AB", "BA").set_matching_strategy(MatchingStrategy::First);
        assert_eq!(rule.apply("ABABAB"), Some("BAABAB".to_owned()));

        let rule = Rule::new("AB", "BA").set_matching_strategy(MatchingStrategy::Last);
        assert_eq!(rule.apply("ABABAB"), Some("ABABBA".to_owned()));

        let rule = Rule::new("AB", "BA").set_matching_strategy(MatchingStrategy::Random);
        assert!([
            Some("BAABAB".to_owned()),
            Some("ABABBA".to_owned()),
            Some("ABBAAB".to_owned())
        ]
        .contains(&rule.apply("ABABAB")));

        // Overlaping patterns
        let rule = Rule::new("ABA", "BAB").set_matching_strategy(MatchingStrategy::Random);
        assert!(
            [Some("BABBAB".to_owned()), Some("ABBABB".to_owned()),].contains(&rule.apply("ABABAB"))
        );
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
