// Notes:
// Playing around with Collatz Sequence.
// This deals with a sequence that involves computing a sequence defined by the following equation:
//   f(n) = n / 2 if n is even
//   f(n) = 3n + 1 if n is odd
//
// applying this function starting at an initial positive integer leads to a sequence of numbers,
// which will eventually reach 1, after which it stabilizes in the pattern 1 - 4 - 2 - 1.
//
// The conjecture that the sequence will always reach 1 is as yet unproven, and remains an open
// problem in Mathematics.

use memoize::memoize;


fn get_next_num(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn compute_sequence(start: u64) -> Vec<u64> {
    let mut seq = Vec::new();
    let mut curr = start;
    seq.push(curr);
    while curr != 1 {
        curr = get_next_num(curr);
        seq.push(curr);
    }
    seq
}

fn compute_sequence_length(start: u64) -> (u64, usize) {
    (start, compute_sequence(start).len() - 1)
}

fn brute_max_collatz_sequence_length(limit: u64) -> (u64, usize) {
    let mut max_result: (u64, usize) = compute_sequence_length(1);
    for i in 2..limit {
        let result = compute_sequence_length(i);
        if result.1 > max_result.1 {
            max_result = result
        }
    }
    max_result
}

#[memoize]
fn collatz_sequence_length(num: u64) -> u64 {
    // attempt to store results of computations to be used as cached values when the function is
    // called with the same args. This seems to marginally improve performance when the limit is
    // 1 million.
    if num == 1 {
        return 0;
    }
    let next = get_next_num(num);
    1 + collatz_sequence_length(next)
}


fn max_collatz_sequence_length(limit: u64) -> (u64, u64) {
    let mut max_result: (u64, u64) = (1, collatz_sequence_length(1));
    for i in 2..limit {
        let len = collatz_sequence_length(i);
        if len > max_result.1 {
            max_result = (i, len);
        }
    }
    max_result

}


fn main() {
    // Compute number with largest Collatz orbit length, up to a certain limit.

    // // Brute Force
    // let limit: u64 = 1_000_000 + 1;
    // let max_result = brute_max_collatz_sequence_length(limit);
    // println!("{:?}", max_result);

    // // Memoization
    // let limit_m: u64 = 1_000_000 + 1;
    // let max_result_m = max_collatz_sequence_length(limit_m);
    // println!("{:?}", max_result_m);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_num() {
        assert_eq!(get_next_num(1), 4);
        assert_eq!(get_next_num(4), 2);
        assert_eq!(get_next_num(2), 1);
        assert_eq!(get_next_num(100), 50);
        assert_eq!(get_next_num(51), 154);
    }

    #[test]
    fn test_compute_sequence() {
        assert_eq!(compute_sequence(1), vec![1]);
        assert_eq!(compute_sequence(2), vec![2, 1]);
        assert_eq!(compute_sequence(4), vec![4, 2, 1]);
        assert_eq!(compute_sequence(10), vec![10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_compute_sequence_length() {
        assert_eq!(compute_sequence_length(1), (1, 1));
        assert_eq!(compute_sequence_length(2), (2, 2));
        assert_eq!(compute_sequence_length(4), (4, 3));
        assert_eq!(compute_sequence_length(10), (10, 7));
        assert_eq!(compute_sequence_length(1_000_000), (1_000_000, 153));
    }

    #[test]
    fn test_collatz_sequence_length0() {
        assert_eq!(collatz_sequence_length(1), 0);
    }

    #[test]
    fn test_collatz_sequence_length1() {
        assert_eq!(collatz_sequence_length(2), 1);
    }

    #[test]
    fn test_collatz_sequence_length2() {
        assert_eq!(collatz_sequence_length(4), 2);
    }

    #[test]
    fn test_collatz_sequence_length3() {
        assert_eq!(collatz_sequence_length(10), 6);
    }

    #[test]
    fn test_collatz_sequence_length4() {
        assert_eq!(collatz_sequence_length(1_000_000), 152);
    }
}