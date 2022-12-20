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

    /// Calculates the index of the start packet (4 consecutive non-repeating
    /// characters), returning [None] if no such index exists.
    pub(crate) fn start_packet_index(&self) -> Option<usize> {
        let mut count_by_char: HashMap<char, usize> = HashMap::new();

        for i in 0..self.0.len() {
            let curr = self.0[i];

            count_by_char.insert(curr, count_by_char.get(&curr).unwrap_or(&0) + 1);

            if count_by_char.len() == 4 {
                return Some(i + 1);
            }

            if i >= 3 {
                let prev = self.0[i - 3];

                let count = count_by_char.get(&prev).unwrap_or(&1) - 1;
                if count <= 0 {
                    count_by_char.remove(&prev);
                }
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
            DataStream(vec!['b', 'v', 'w', 'b', 'j', 'p', 'l']).start_packet_index(),
            Some(5)
        );

        assert_eq!(
            DataStream(vec!['a', 'a', 'a', 'b', 'c', 'd', 'e']).start_packet_index(),
            Some(6)
        );

        assert_eq!(
            DataStream(vec!['a', 'a', 'a', 'b', 'c', 'c', 'c']).start_packet_index(),
            None
        );
    }
}
