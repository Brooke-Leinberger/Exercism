use std::collections::HashSet;

fn get_factors(limit: u32, factor: u32) -> Vec<u32>
{
    let max : u32 = (limit + factor - 1) / factor;
    (1..max).map(|c| factor * c).collect::<Vec<u32>>()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut mults = HashSet::<u32>::new();
    for &factor in factors.iter().filter(|&&c| c != 0) {mults.extend(get_factors(limit, factor));}
    mults.iter().sum()
}
