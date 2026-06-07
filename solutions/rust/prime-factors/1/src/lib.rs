use std::collections::VecDeque;

pub fn sieve(n: u64) -> Vec<u64>
{
    let mut primes: Vec<u64> = Vec::new();
    let mut nums: VecDeque<u64> = (2..(n + 1)).collect();

    while !nums.is_empty()
    {
        let n : u64 = nums.pop_front().unwrap();
        primes.push(n);
        nums.retain(|c| c % n != 0);
    }

    primes
}

pub fn recursive(n: u64, primes: &Vec<u64>) -> VecDeque<u64>
{
    let first : Option<usize> = primes.iter().position(|p| n % p == 0);
    if first.is_none() { return VecDeque::<u64>::new(); }

    let mut list : VecDeque<u64>;
    let prime = primes[first.unwrap()];
    let quot = n / prime;
    if quot != 1 {list = recursive(quot, primes)} else {list = VecDeque::<u64>::new();}
    list.push_front(prime);
    println!("Recursive({:?}): {:?}", n, list);

    list
}

pub fn factors(n: u64) -> Vec<u64> {
    let primes = sieve(n);
    Vec::<u64>::from(recursive(n, &primes))
}
