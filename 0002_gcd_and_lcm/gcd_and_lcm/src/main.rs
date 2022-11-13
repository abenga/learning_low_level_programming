
// Write a program that, given two positive integers, will calculate and print the greatest
// common divisor of the two.

// Write a program that will, given two or more positive integers, calculate and print the least
// common multiple of them all.


fn gcd(a: u32, b: u32) -> u32 {
    let (mut m, mut n) = if a > b { (a, b) } else { (b, a) };
    let mut r = m % n;
    while r > 0 {
        m = n;
        n = r;
        r = m % n;
    }
    return n;
}


fn lcm(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}


fn main() {
    println!("gcd(2, 4) = {}", gcd(2, 4));
    println!("gcd(3, 6) = {}", gcd(3, 6));
    println!("gcd(7, 124) = {}", gcd(7, 124));
    println!("gcd(270, 192) = {}", gcd(270, 192));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_0() {
        assert_eq!(gcd(2, 6), 2);
    }

    #[test]
    fn test_gcd_1() {
        assert_eq!(gcd(7, 19), 1);
    }

    #[test]
    fn test_lcm_0() {
        assert_eq!(lcm(2, 8), 8);
    }

    #[test]
    fn test_lcm_1() {
        assert_eq!(lcm(3, 11), 33);
    }

}
