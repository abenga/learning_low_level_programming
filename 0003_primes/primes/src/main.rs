
fn get_digits(a: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    let mut t = a;

    while t > 0 {
        digits.push(t % 10);
        t = t / 10;
    }
    digits
}


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


fn get_proper_divisors(num: u32) -> Vec<u32> {
    let mut divisors: Vec<u32> = Vec::new();
    if num == 1 {
        return divisors;
    }
    divisors.push(1);
    let limit = num ^ ( 1 / 2);
    for i in 2..limit {
        if num % i == 0 {
            divisors.push(i)
        }
    }
    divisors
}

// Modern C++ Challenge, Cap 1, Q4
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

// Modern C++ Challenge, Cap 1, Q5
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

// Modern C++ Challenge, Cap 1, Q9
// Write a program that returns the prime factors of a number entered by the user
fn get_prime_factors(num: u32) -> Vec<u32> {
    let divisors = get_proper_divisors(num);
    let mut factors: Vec<u32> = Vec::new();
    for divisor in divisors {
        if !is_prime(divisor) {
            continue;
        }
        factors.push(divisor);
    }
    factors
}

// Modern C++ Challenge, Cap 1, Q9
// Write a program that prints all abundant numbers and their abundance, up to a number entered by
// the user.
//
// Abundant numbers are numbers where the sum of all proper divisors is greater than the number.
fn get_abundant_numbers(limit: u32) -> Vec<(u32, u32)> {
    let mut abundant_numbers: Vec<(u32, u32)> = Vec::new();

    for i in 1..limit {
        let divisors = get_proper_divisors(i);
        let sum_of_divisors: u32 = divisors.iter().sum();
        if sum_of_divisors > i {
            abundant_numbers.push((i, sum_of_divisors-i));
        }
    }
    abundant_numbers
}

// Modern C++ Challenge, Cap 1, Q8
// Write a program that prints all Armstrong numbers with three digits.
//
// An Armstrong number a number that is equal to the sum of its own digits when they are raised to
// the power of the number of digits.
fn get_armstrong_numbers() -> Vec<u32> {
    let mut armstrong_nums: Vec<u32> = Vec::new();
    for i in 100..1000 {
        let digits = get_digits(i);
        let power_sum = digits[0] * digits[0] * digits[0] +
            digits[1] * digits[1] * digits[1] +
            digits[2] * digits[2] * digits[2];
        if power_sum == i {
            armstrong_nums.push(i);
        }
    }
    armstrong_nums
}


fn main() {
    // println!("Please submit the limit (positive integer):");
    // let mut limit = String::new();
    //
    // io::stdin().read_line(&mut limit).expect("Failed to read limit");
    //
    // let limit: u32 = limit.trim().parse().expect("Expected a positive integer!");
    //
    // let a: u32 = 25;
    // println!("is_prime({}) = {}", a, is_prime(a));
    //
    // let limit: u32 = 10_000;
    //
    // let factors = print_sexy_primes_less_than_limit(limit);
    // println!("There were {} pairs found", pairs.len());
    // for pair in pairs {
    //     println!("{:?}", pair);
    // }
    //
    // let num: u32 = 10_000;
    //
    // let factors = get_prime_factors(num);
    // println!("get_prime_factors({}) = {:?}", num, factors);
    //
    // let limit: u32 = 20;
    //
    // let abundant_numbers = get_abundant_numbers(limit);
    // println!("get_abundant_numbers({}) = {:?}", limit, abundant_numbers);

    let nums = get_armstrong_numbers();
    println!("{:#?}", nums)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_digits() {
        assert_eq!(get_digits(1), vec![1]);
        assert_eq!(get_digits(2), vec![2]);
        assert_eq!(get_digits(10), vec![0, 1]);
        assert_eq!(get_digits(15), vec![5, 1]);
        assert_eq!(get_digits(100), vec![0, 0, 1]);
        assert_eq!(get_digits(123), vec![3, 2, 1]);

    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(16), false);
        assert_eq!(is_prime(25), false);
        assert_eq!(is_prime(31), true);
    }

    #[test]
    fn test_largest_prime_less_than_limit() {
        assert_eq!(largest_prime_less_than_limit(1), 0);
        assert_eq!(largest_prime_less_than_limit(2), 2);
        assert_eq!(largest_prime_less_than_limit(154), 151);
        assert_eq!(largest_prime_less_than_limit(4_093), 4_093);
        assert_eq!(largest_prime_less_than_limit(5_000_000), 4_999_999);
    }

    #[test]
    fn test_print_sexy_primes_less_than_limit() {
        let empty_vec: Vec<Vec<u32>> = Vec::new();
        assert_eq!(print_sexy_primes_less_than_limit(10), empty_vec);

        assert_eq!(
            print_sexy_primes_less_than_limit(15),
            vec![vec![5, 11], vec![7, 13]]
        );

        assert_eq!(
            print_sexy_primes_less_than_limit(20),
            vec![vec![5, 11], vec![7, 13], vec![11, 17], vec![13, 19]]
        );
    }

    #[test]
    fn test_get_prime_factors() {
        let empty_vec: Vec<u32> = Vec::new();
        assert_eq!(get_prime_factors(3), empty_vec);

        assert_eq!(get_prime_factors(512), vec![2]);

        assert_eq!(get_prime_factors(15), vec![3, 5]);

        assert_eq!(get_prime_factors(1_547_123), vec![53, 29191]);

        assert_eq!(get_prime_factors(19_090_779), vec![3, 17, 37, 67, 151]);
    }

    #[test]
    fn test_get_proper_divisors() {
        let empty_vec: Vec<u32> = Vec::new();
        assert_eq!(get_proper_divisors(1), empty_vec);

        assert_eq!(get_proper_divisors(2), vec![1]);

        assert_eq!(get_proper_divisors(3), vec![1]);

        assert_eq!(get_proper_divisors(4), vec![1, 2]);

        assert_eq!(get_proper_divisors(6), vec![1, 2, 3]);

        assert_eq!(get_proper_divisors(10), vec![1, 2, 5]);

        assert_eq!(get_proper_divisors(12), vec![1, 2, 3, 4, 6]);

        assert_eq!(get_proper_divisors(512), vec![1, 2, 4, 8, 16, 32, 64, 128, 256]);

        assert_eq!(get_proper_divisors(19_090_779),
                   vec![
                       1, 3, 17, 37, 51, 67, 111, 151, 201, 453, 629, 1139, 1887, 2479, 2567,
                       3417, 5587, 7437, 7701, 10117, 16761, 30351, 42143, 94979, 126429,
                       171989, 284937, 374329, 515967, 1122987, 6363593
                   ]);
    }

    #[test]
    fn test_get_abundant_numbers() {
        let empty_vec: Vec<(u32, u32)> = Vec::new();
        assert_eq!(get_abundant_numbers(3), empty_vec);

        assert_eq!(get_abundant_numbers(20), vec![(12, 4), (18, 3)]);

        assert_eq!(
            get_abundant_numbers(40),
            vec![(12, 4), (18, 3), (20, 2), (24, 12), (30, 12), (36, 19)]
        );
    }

}