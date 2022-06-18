use markov_runner::nodes::{RandomChoice, Rule, Sequence};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Algorithm {
    BinaryToUnary,
    RandomMarch,
}

pub fn binary_to_unary_node() -> Sequence {
    Sequence::new()
        .add_node(Rule::new("1", "0x"))
        .add_node(Rule::new("x0", "0xx"))
        .add_node(Rule::new("0", ""))
}

pub fn random_march() -> Sequence {
    Sequence::new().add_node(Box::new(
        RandomChoice::new()
            .add_node(Rule::new("OXO", "OOX"))
            .add_node(Rule::new("OXO", "XOO")),
    ))
}
