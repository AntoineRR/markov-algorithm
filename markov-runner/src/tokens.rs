pub trait Token {
    fn apply(&self, input: &str) -> Option<String>;
}

#[derive(Debug, Default)]
pub struct Sequence<T: Token> {
    tokens: Vec<T>,
}

impl<T: Token> Sequence<T> {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn add_rule(mut self, token: T) -> Self {
        self.tokens.push(token);
        self
    }
}

impl<T: Token> Token for Sequence<T> {
    fn apply(&self, input: &str) -> Option<String> {
        for token in &self.tokens {
            let result = token.apply(input);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

#[derive(Debug)]
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

impl Token for Rule {
    fn apply(&self, input: &str) -> Option<String> {
        let mut result = input.to_owned();
        if let Some(index) = input.find(&self.input) {
            result.replace_range(index..index + self.input.len(), &self.output);
            return Some(result);
        }
        None
    }
}
