use std::collections::VecDeque;

pub fn find_start_of_sequence_position(sequence: &str, start_len: usize) -> Option<u32> {
    let mut start_of_packet: VecDeque<char> = VecDeque::with_capacity(start_len);
    for (idx, item) in sequence.chars().enumerate() {
        while start_of_packet.contains(&item) {
            start_of_packet.pop_front();
        }
        start_of_packet.push_back(item);

        if start_of_packet.len() == start_len {
            return Some((idx + 1) as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::super::file;
    use super::*;

    #[test]
    fn return_position_5_for_test_input() {
        assert_eq!(
            5,
            find_start_of_sequence_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap()
        );
    }

    #[test]
    fn return_position_6_for_test_input() {
        assert_eq!(
            6,
            find_start_of_sequence_position("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap()
        );
    }

    #[test]
    fn return_position_10_for_test_input() {
        assert_eq!(
            10,
            find_start_of_sequence_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap()
        );
    }

    #[test]
    fn return_position_11_for_test_input() {
        assert_eq!(
            11,
            find_start_of_sequence_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap()
        );
    }
}
