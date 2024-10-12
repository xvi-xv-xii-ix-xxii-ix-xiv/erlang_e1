// Erlang E1 Channels Calculation Library without external dependencies.

/// Calculates the blocking probability using the Erlang B formula.
/// This iterative approach sums up traffic for every available channel.
///
/// # Arguments
/// * `traffic` - The traffic load in Erlangs.
/// * `channels` - The number of communication channels.
///
/// # Returns
/// The probability that all channels are busy (blocking probability).
pub fn erlang_b(traffic: f64, channels: u32) -> f64 {
    let mut inverse_b = 1.0;

    for n in 1..=channels {
        inverse_b = 1.0 + inverse_b * (n as f64 / traffic);
    }

    1.0 / inverse_b
}

/// Iteratively calculates the number of E1 voice channels required to satisfy
/// a given blocking probability using the Erlang B formula.
///
/// # Arguments
/// * `traffic` - The traffic load in Erlangs.
/// * `blocking_probability` - Desired blocking probability (between 0 and 1).
/// * `channels_max` - Maximum number of channels to search for (default 10,000).
///
/// # Returns
/// Returns the number of channels required to meet the blocking probability, or `None`
/// if the number of channels exceeds `channels_max`.
pub fn calculate_e1_channels(
    traffic: f64,
    blocking_probability: f64,
    channels_max: u32,
) -> Option<u32> {
    let mut channels = 1;

    while channels < channels_max {
        let blocking = erlang_b(traffic, channels);
        if blocking <= blocking_probability {
            return Some(channels);
        }
        channels += 1;
    }

    None
}

/// Calculates the required number of E1 voice channels for a given number of users,
/// average call duration, concurrent calls, and desired blocking probability.
///
/// This is a helper function to convert user input into traffic (Erlangs) and then
/// call `calculate_e1_channels` for the actual channel computation.
///
/// # Arguments
/// * `users` - Number of users.
/// * `average_call_duration` - Average call duration in minutes.
/// * `concurrent_calls` - Number of simultaneous calls.
/// * `blocking_probability` - Desired blocking probability.
///
/// # Returns
/// Number of required voice channels, or `None` if the search exceeds `channels_max`.
pub fn required_e1_channels(
    users: u32,
    average_call_duration: f64,
    concurrent_calls: u32,
    blocking_probability: f64,
) -> Option<u32> {
    let traffic = (users as f64 * average_call_duration * concurrent_calls as f64) / 60.0;
    calculate_e1_channels(traffic, blocking_probability, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erlang_b() {
        let traffic = 20.0;
        let channels = 10;
        let blocking_prob = erlang_b(traffic, channels);
        assert!(blocking_prob > 0.0);
        assert!(blocking_prob < 1.0);
    }

    #[test]
    fn test_calculate_e1_channels() {
        let traffic = 15.0;
        let blocking_probability = 0.05;
        let channels = calculate_e1_channels(traffic, blocking_probability, 100);
        assert!(channels.is_some());
        assert!(channels.unwrap() >= 1);
    }

    #[test]
    fn test_required_e1_channels() {
        let users = 100;
        let average_call_duration = 3.0; // 3 minutes
        let concurrent_calls = 10;
        let blocking_probability = 0.05;
        let channels = required_e1_channels(
            users,
            average_call_duration,
            concurrent_calls,
            blocking_probability,
        );
        assert!(channels.is_some());
    }
}
