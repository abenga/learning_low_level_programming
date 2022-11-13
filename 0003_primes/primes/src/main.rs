
use std::io;


fn is_prime(a: u32) -> bool {
    return if a <= 3 {
        a > 1
    } else if a % 2 == 0 || a % 3 == 0 {
        false
    } else {
        let mut i: u32 = 5;
        while i * i < a {
            if a % i == 0 || a % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}


fn largest_prime_less_than_limit(lim: u32) -> u32 {
    let mut i = lim;
    while i > 1 {
        println!("i = {}, is_prime({}) = {}", i, i, is_prime(i));
        if is_prime(i) {
            return i;
        }
        i -= 1;
    }
    return 0;
}


fn main() {
    // println!("Please submit the limit (positive integer):");
    // let mut limit = String::new();
    //
    // io::stdin().read_line(&mut limit).expect("Failed to read limit");
    //
    // let limit: u32 = limit.trim().parse().expect("Expected a positive integer!");

    let limit: u32 = 2;

    println!("largest_prime_less_than_limit({}) = {}", limit, largest_prime_less_than_limit(limit));


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn test_largest_prime_less_than_limit() {
        assert_eq!(largest_prime_less_than_limit(1), 0);
        assert_eq!(largest_prime_less_than_limit(2), 2);
        assert_eq!(largest_prime_less_than_limit(154), 151);
    }

}