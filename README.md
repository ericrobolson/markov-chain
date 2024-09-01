# markov-chain

A simple implementation of a Markov chain in Rust.

## Installation

Add to your `Cargo.toml` file dependencies:

```toml
[dependencies]
markov-chain = { git = "https://github.com/ericrobolson/markov-chain.git" }
```

## Usage

```rust
use markov_chain::MarkovChain;

let window_size = 3;
let words = vec!["I", "am", "a", "test", "sentence.", "I", "is", "happy"];

let chain = MarkovChain::new(window_size, words);

let mut buffer = vec!["I"];
if let Some(next) = chain.generate(&buffer){
    buffer.push(next);
}
println!("{:?}", buffer);
```

See `examples/poem.rs` for a more complete example that randomly outputs text from a file.
