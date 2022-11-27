
fn is_prime(a: u32) -> bool {
    return if a <= 3 {
        a > 1
    } else if a % 2 == 0 || a % 3 == 0 {
        false
    } else {
        let mut i: u32 = 5;
        while i * i <= a {
            if a % i == 0 || a % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}


// Write a program that computes and prints the largest prime number that is
// smaller than a number provided by the user, which must be a positive integer.
fn largest_prime_less_than_limit(lim: u32) -> u32 {
    let mut i = lim;
    while i > 1 {
        println!("i = {}, is_prime({}) = {}", i, i, is_prime(i));
        if is_prime(i) {
            return i;
        }
        i -= 1;
    }
    0
}


// Write a program that prints all the sexy prime pairs up to a limit entered by the user
fn print_sexy_primes_less_than_limit(lim: u32) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();
    for i in 2..(lim - 6) {
        if is_prime(i) && is_prime(i + 6) {
            let entry = vec![i, i+6];
            res.push(entry);
        }
    }
    res
}


// Write a program that returns the prime factors of a number entered by the user
fn get_prime_factors(num: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = Vec::new();
    let max_to_check = num ^ (1 / 2);

    for i in 2..max_to_check {
        if !is_prime(i) {
            continue;
        }
        if num % i == 0 {
            factors.push(i);
        }
    }
    factors
}


fn main() {
    // println!("Please submit the limit (positive integer):");
    // let mut limit = String::new();
    //
    // io::stdin().read_line(&mut limit).expect("Failed to read limit");
    //
    // let limit: u32 = limit.trim().parse().expect("Expected a positive integer!");

    let a: u32 = 25;
    println!("is_prime({}) = {}", a, is_prime(a));

    // let limit: u32 = 10_000;
    //
    // let factors = print_sexy_primes_less_than_limit(limit);
    // println!("There were {} pairs found", pairs.len());
    // for pair in pairs {
    //     println!("{:?}", pair);
    // }

    let num: u32 = 10_000;

    let factors = get_prime_factors(num);
    println!("get_prime_factors({}) = {:?}", num, factors);

}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_is_prime() {
    //     assert_eq!(is_prime(1), false);
    //     assert_eq!(is_prime(2), true);
    //     assert_eq!(is_prime(3), true);
    //     assert_eq!(is_prime(6), false);
    //     assert_eq!(is_prime(4), false);
    //     assert_eq!(is_prime(16), false);
    //     assert_eq!(is_prime(25), false);
    //     assert_eq!(is_prime(31), true);
    // }
    //
    // #[test]
    // fn test_largest_prime_less_than_limit() {
    //     assert_eq!(largest_prime_less_than_limit(1), 0);
    //     assert_eq!(largest_prime_less_than_limit(2), 2);
    //     assert_eq!(largest_prime_less_than_limit(154), 151);
    //     assert_eq!(largest_prime_less_than_limit(4_093), 4_093);
    //     assert_eq!(largest_prime_less_than_limit(5_000_000), 4_999_999);
    // }
    //
    // #[test]
    // fn test_print_sexy_primes_less_than_limit() {
    //     let empty_vec: Vec<Vec<u32>> = Vec::new();
    //     assert_eq!(print_sexy_primes_less_than_limit(10), empty_vec);
    //
    //     assert_eq!(
    //         print_sexy_primes_less_than_limit(15),
    //         vec![vec![5, 11], vec![7, 13]]
    //     );
    //
    //     assert_eq!(
    //         print_sexy_primes_less_than_limit(20),
    //         vec![vec![5, 11], vec![7, 13], vec![11, 17], vec![13, 19]]
    //     );
    // }

    #[test]
    fn test_get_prime_factors() {
        let empty_vec: Vec<u32> = Vec::new();
        assert_eq!(get_prime_factors(3), empty_vec);

        assert_eq!(get_prime_factors(512), vec![2]);

        assert_eq!(get_prime_factors(15), vec![3, 5]);

        assert_eq!(get_prime_factors(1547123), vec![53, 29191]);

        assert_eq!(get_prime_factors(19_090_779), vec![3, 17, 37, 67, 151]);


        // assert_eq!(
        //     get_prime_factors(20),
        //     vec![vec![5, 11], vec![7, 13], vec![11, 17], vec![13, 19]]
        // );
    }

}