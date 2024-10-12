# Erlang E1 Channel Calculator

This Rust library provides functionality to calculate the number of E1 voice channels required to meet a specific blocking probability using the Erlang B formula. It is commonly used in telecommunications to determine the number of channels needed based on system traffic and user load.

## Features

- **Erlang B Calculation**: Calculate the blocking probability based on traffic (in Erlangs) and the number of available communication channels.
- **E1 Channel Calculation**: Compute the number of E1 voice channels required to meet a desired blocking probability.
- **Helper Functions**: Convert high-level user inputs such as the number of users, average call duration, and concurrent calls into Erlangs and perform the channel calculation.

Then import the library into your project:

```rust
extern crate erlang_e1;
```

Usage

Calculating Blocking Probability
You can calculate the blocking probability for a given traffic load and number of channels using the Erlang B formula:

```rust
use erlang_e1::erlang_b;

fn main() {
    let traffic = 15.0; // in Erlangs
    let channels = 10;
    let blocking_probability = erlang_b(traffic, channels);
    println!("Blocking Probability: {:.5}", blocking_probability);
}
```

Calculating Required E1 Channels
If you already have the traffic (in Erlangs), you can calculate the number of E1 channels needed for a desired blocking probability:

```rust

use erlang_e1::calculate_e1_channels;

fn main() {
    let traffic = 20.0; // in Erlangs
    let blocking_probability = 0.05; // 5% blocking probability
    let max_channels = 10000; // optional upper limit for search

    match calculate_e1_channels(traffic, blocking_probability, max_channels) {
        Some(channels) => println!("Required channels: {}", channels),
        None => println!("No suitable number of channels found within the limit."),
    }
}
```

High-Level Calculation with User Inputs
If you have high-level inputs like the number of users and average call duration, you can use the required_e1_channels
function to calculate the number of channels:

```rust

use erlang_e1::required_e1_channels;

fn main() {
    let users = 100;
    let average_call_duration = 3.0; // in minutes
    let concurrent_calls = 10;
    let blocking_probability = 0.05;

    match required_e1_channels(users, average_call_duration, concurrent_calls, blocking_probability) {
        Some(channels) => println!("Required channels: {}", channels),
        None => println!("No suitable number of channels found within the limit."),
    }
}
```

Explanation
Erlang B Formula: This formula is used to calculate the probability of all channels being occupied (blocking probability) in a system with N channels and a given traffic load in Erlangs.
E1 Channels: In telecommunications, an E1 line consists of 30 voice channels. This library helps calculate the number of E1 lines required to satisfy the traffic and blocking probability requirements.
