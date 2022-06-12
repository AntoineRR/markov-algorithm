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
