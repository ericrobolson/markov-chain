use rand::seq::SliceRandom;
use std::collections::HashMap;

/// A Markov chain implementation.
#[derive(Clone)]
pub struct MarkovChain<State>
where
    State: Eq + std::hash::Hash + Clone,
{
    chains: HashMap<Vec<State>, Vec<State>>,
    max_chain_length: usize,
}

impl<State> MarkovChain<State>
where
    State: Eq + std::hash::Hash + Clone,
{
    /// Create a new MarkovChain over the given input with a maximum chain length.
    pub fn new(max_chain_length: usize, input: Vec<State>) -> Self {
        let mut chains = HashMap::new();
        // Iterate over input and create chains.
        // We do this up to the window size to allow more generation options.
        for iteration in 0..max_chain_length {
            for window in input.windows(iteration + 1) {
                let state = window[0..iteration].to_vec();
                let next_state = window[iteration].clone();
                chains
                    .entry(state)
                    .or_insert_with(Vec::new)
                    .push(next_state);
            }
        }

        MarkovChain {
            max_chain_length,
            chains,
        }
    }

    /// Generate a new state based on the given state.
    pub fn generate(&self, state: &Vec<State>) -> Option<&State> {
        self.generate_with_length(self.max_chain_length, state)
    }

    /// Generate a new state based on the given state.
    pub fn generate_with_length(
        &self,
        max_chain_length: usize,
        state: &Vec<State>,
    ) -> Option<&State> {
        // Ensure that the given length is not greater than the maximum calculated chain length.
        let mut max_chain_length = self.max_chain_length.min(max_chain_length);
        // Iterate over the state to find a prediction.
        // Decrease window size if nothing was found to allow more
        // generation options.
        while max_chain_length > 0 {
            let idx = if state.len() < max_chain_length {
                0
            } else {
                state.len() - max_chain_length
            };

            let state: Vec<State> = state[idx..].to_vec();

            let predicted = self
                .chains
                .get(&state)
                .and_then(|v| v.choose(&mut rand::thread_rng()));

            if predicted.is_some() {
                return predicted;
            }

            // Decrease window size to allow more generation options.
            max_chain_length -= 1;
        }

        None
    }
}
