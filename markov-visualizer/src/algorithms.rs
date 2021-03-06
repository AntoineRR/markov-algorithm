use markov_runner::nodes::{RandomChoice, Rule, Sequence};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Algorithm {
    BinaryToUnary,
    RandomMarch,
}

pub fn binary_to_unary_node() -> Sequence {
    Sequence::new()
        .add_node(Rule::boxed("1", "0x"))
        .add_node(Rule::boxed("x0", "0xx"))
        .add_node(Rule::boxed("0", ""))
}

pub fn random_march() -> RandomChoice {
    RandomChoice::new()
        .add_node(Rule::boxed("OXO", "OOX"))
        .add_node(Rule::boxed("OXO", "XOO"))
}
