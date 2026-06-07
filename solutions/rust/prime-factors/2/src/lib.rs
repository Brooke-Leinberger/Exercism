use std::collections::VecDeque;

pub fn sieve(n: u64) -> Vec<u64>
{
    let mut primes: Vec<u64> = Vec::new();
    let mut nums: VecDeque<u64> = (2..(n + 1)).collect();

    while !nums.is_empty()
    {
        let n : u64 = nums.pop_front().unwrap();
        primes.push(n);
        nums.retain(|c| !c.is_multiple_of(n));
    }

    primes
}

pub fn recursive(n: u64, primes: &Vec<u64>) -> VecDeque<u64>
{
    let mut list = VecDeque::<u64>::new();
    let first : Option<usize> = primes.iter().position(|p| n.is_multiple_of(*p));
    if first.is_none() { return list; }

    let prime = primes[first.unwrap()];
    if n != prime {list = recursive(n / prime, primes)}
    list.push_front(prime);

    list
}

pub fn factors(n: u64) -> Vec<u64> {
    let primes = sieve(n);
    Vec::<u64>::from(recursive(n, &primes))
}
