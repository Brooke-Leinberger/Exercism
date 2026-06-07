/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits = code.chars().rev()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    println!("Digits: {:?}", digits);
    
    let trans : Vec<u32> = digits.iter().enumerate()
        .map(|(index, value)| value * (if index % 2 == 0 {1} else {2}))
        .map(|v| if v > 9 {v - 9} else {v}).collect();
    println!("Trans: {:?}", trans);

    let valid = trans.iter().sum::<u32>() % 10 == 0;

    digits.len() > 1 && code.chars().filter(|c| !c.is_whitespace() && !c.is_digit(10)).collect::<Vec<char>>().len() == 0 && valid
}
