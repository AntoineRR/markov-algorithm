# Markov algorithm runner and visualizer

This repository is a test for running Markov algorithms using Rust.

## What are Markov algorithms?

I discovered Markov algorithms through the [Markov Junior](https://github.com/mxgmn/MarkovJunior) project, which has a very nice `README.md` explaining the basis of Markov algorithms.

## Project organisation

The project is separated in a library and a binary crates.

### Markov Runner

This library crate is the core of the Markov algortihm interpreter. Given an input and a set of rules, it computes the result when applying the Markov algorithm resulting from the set of rules to the input.

### Markov Visualizer

This binary crates uses the Markov Runner crate to compute the result of a Markov algorithm, and display it step after step on screen.

## References

- [Markov Junior](https://github.com/mxgmn/MarkovJunior)