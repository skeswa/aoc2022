use std::collections::HashMap;

/// Represents a single stream of incoming data from the elven communication
/// system.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DataStream(Vec<char>);

impl DataStream {
    /// Interprets the given `encoded_data_stream` as an instantiated
    /// [DataStream] object.
    pub(crate) fn parse(encoded_data_stream: &str) -> DataStream {
        DataStream(encoded_data_stream.chars().collect::<Vec<char>>())
    }

    /// Calculates the index of the start of message (14 consecutive
    /// non-repeating characters), returning [None] if no such index exists.
    pub(crate) fn start_of_message_index(&self) -> Option<usize> {
        self.index_after_unique_span(14)
    }

    /// Calculates the index of the start of packet (4 consecutive non-repeating
    /// characters), returning [None] if no such index exists.
    pub(crate) fn start_of_packet_index(&self) -> Option<usize> {
        self.index_after_unique_span(4)
    }

    /// Returns the first index following a `span_length` long sequence of
    /// non-repeating characters, or [None] if no such index exists.
    fn index_after_unique_span(&self, span_length: usize) -> Option<usize> {
        let mut count_by_char: HashMap<char, usize> = HashMap::new();

        for i in 0..self.0.len() {
            let curr = self.0[i];

            count_by_char.insert(curr, count_by_char.get(&curr).unwrap_or(&0) + 1);

            if i >= span_length {
                let prev = self.0[i - span_length];

                let count = count_by_char.get(&prev).unwrap_or(&1) - 1;
                if count > 0 {
                    count_by_char.insert(prev, count);
                } else {
                    count_by_char.remove(&prev);
                }
            }

            if count_by_char.len() == span_length {
                return Some(i + 1);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_reads_data_streams_correctly() {
        assert_eq!(
            DataStream::parse("abcdef"),
            DataStream(vec!['a', 'b', 'c', 'd', 'e', 'f'])
        )
    }

    #[test]
    fn start_packet_index_finds_the_right_start_marker() {
        assert_eq!(
            DataStream(vec![
                'b', 'b', 'b', 'b', 'b', 'b', 'b', 'v', 'w', 'b', 'j', 'p', 'l'
            ])
            .start_of_packet_index(),
            Some(11)
        );

        assert_eq!(
            DataStream(vec!['a', 'a', 'a', 'b', 'c', 'd', 'e']).start_of_packet_index(),
            Some(6)
        );

        assert_eq!(
            DataStream(vec!['a', 'a', 'a', 'b', 'c', 'c', 'c', 'd']).start_of_packet_index(),
            None
        );
    }
}
