/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits : Vec<u32> = code.chars().rev()
        .filter_map(|c| c.to_digit(10)).enumerate()
        .map(|(index, value)| {
            let prod = value * if index % 2 == 0 {1} else {2};
            if prod > 9 {prod - 9} else {prod}
        }).collect();

    digits.len() > 1 && 
    code.chars().filter(|c| !c.is_whitespace() && !c.is_ascii_digit()).collect::<Vec<char>>().is_empty() && 
    digits.iter().sum::<u32>() % 10 == 0
}
