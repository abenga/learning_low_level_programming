
use anyhow::anyhow;
use anyhow::Result;


fn convert_number_to_roman(num: i32) -> Result<String> {
    if num < 0 {
        return Err(anyhow!("Only positive numbers supported!"));
    }
    if num > 3999 {
        return Err(anyhow!("Maximum number representable as Roman numeral is 3999!"));
    }

    let stops = vec!(
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"),
        (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
    );

    let mut out = String::from("");
    let mut current = num;
    let mut i:usize = 0;
    while i < stops.len() {
        println!("{}", i);
        if current >= stops[i].0 {
            out.push_str(stops[i].1);
            current -= stops[i].0;
        } else {
            i += 1;
        }
    }
    Ok(out)
}


fn main() {
    let mut ans = convert_number_to_roman(1);
    println!("{:?}", ans);
    ans = convert_number_to_roman(5000);
    println!("{:?}", ans);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_number_to_roman_0() {
        assert_eq!(convert_number_to_roman(1).unwrap(), String::from("I"));
        assert_eq!(convert_number_to_roman(2).unwrap(), String::from("II"));
        assert_eq!(convert_number_to_roman(4).unwrap(), String::from("IV"));
        assert_eq!(convert_number_to_roman(50).unwrap(), String::from("L"));
        assert_eq!(convert_number_to_roman(99).unwrap(), String::from("XCIX"));
        assert_eq!(convert_number_to_roman(2022).unwrap(), String::from("MMXXII"));
    }
}
