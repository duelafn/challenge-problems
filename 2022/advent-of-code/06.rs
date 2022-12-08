
use std::collections::HashSet;
use std::iter::FromIterator;


fn start_of_packet(v: &[char], n: usize) -> usize {
    for (idx, w) in v.windows(n).enumerate() {
        if HashSet::<char>::from_iter(w.into_iter().copied()).len() == n { return n + idx; }
    }
    panic!("Bummer");
}


fn main() {
    let vec = std::fs::read_to_string("06.in").unwrap().chars().collect::<Vec<_>>();
    println!("Part 1: {}", start_of_packet(&vec, 4));
    println!("Part 2: {}", start_of_packet(&vec, 14));
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<_>>().as_slice(), 4), 5);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg".chars().collect::<Vec<_>>().as_slice(), 4), 6);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect::<Vec<_>>().as_slice(), 4), 10);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect::<Vec<_>>().as_slice(), 4), 11);

        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect::<Vec<_>>().as_slice(), 14), 23);
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg".chars().collect::<Vec<_>>().as_slice(), 14), 23);
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect::<Vec<_>>().as_slice(), 14), 29);
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect::<Vec<_>>().as_slice(), 14), 26);
    }
}
