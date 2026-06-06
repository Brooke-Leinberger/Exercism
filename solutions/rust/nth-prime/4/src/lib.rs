use std::collections::*;

pub fn sieve_pass(prime: u32, sieve: &mut VecDeque<u32>, min: u32, max: u32)
{
    let comps: HashSet<u32> = ((min/prime)..(max/prime + 1))
        .map(move |s| prime * s)
        .collect();

    for comp in comps 
    {
        if let Some(pos) = sieve.iter().position(|&x| x == comp) { sieve.remove(pos); }
    }
}

pub fn nth(n: u32) -> u32 {
    let chunk = 100;
    let mut stream = 2..;
    
    let mut primes : Vec<u32> = Vec::new();
    let mut sieve : VecDeque<u32> = VecDeque::new();

    let mut min    = 0;
    let mut max    = 0;
    
    while primes.len() < (n + 1) as usize
    {
        if sieve.is_empty()
        {
            sieve = (&mut stream).take(chunk).collect::<VecDeque<u32>>();
            min = *sieve.front().unwrap();
            max = *sieve.back().unwrap();

            for prime in &primes {sieve_pass(*prime, &mut sieve, min, max);}
        }
         
        let pop : u32 = sieve.pop_front().unwrap();
        
        primes.push(pop);
        sieve_pass(pop, &mut sieve, min, max);
    }

    *primes.last().unwrap()
}