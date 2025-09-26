pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {return true;}
    let digits: u32 = num.ilog10() + 1;

    let mut sum = 0;
    for digit in 0..digits { sum += ((num / 10_u32.pow(digit)) % 10).pow(digits); }
    sum == num
}
