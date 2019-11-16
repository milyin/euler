use std::io;
use std::io::Write;

struct Sieve {
    sieve: Vec<bool>,
    primes: Vec<i64>,
    next: i64,
}

impl Default for Sieve {
   fn default() -> Sieve {
       Sieve {
           sieve: vec![true; 100000000],
           primes: Vec::new(),
           next: 3,
       }
   }
}

impl Sieve {
    fn n2pos(n:i64) -> usize { (( n - 3 ) / 2) as usize }
    //fn pos2n(p:usize) -> i64 { (p * 2 + 3) as i64 }
    fn tap_sieve(sieve: &mut Vec<bool>, n: i64) {
        if n % 2 != 0 {
            sieve[Sieve::n2pos(n)] = false;
        }
    }
    fn try_next_prime(&mut self) -> bool {
        let candidate = self.next;
        self.next += 2;
        if Sieve::n2pos(candidate) < self.sieve.len() {
            if self.sieve[Sieve::n2pos(candidate)] { // if prime
                self.primes.push(candidate);
                let mut next_multiple = candidate * candidate;
                while Sieve::n2pos(next_multiple) < self.sieve.len() {
                    Sieve::tap_sieve(&mut self.sieve, next_multiple);
                    next_multiple += candidate;
                }
                true
            } else {
                false
            }
        } else { // gone out of sieve - fallback to factoring each value
            for factor in &self.primes {
                if candidate % factor == 0 {
                    return false;
                }
            }
            self.primes.push(candidate);
            true
        }
    }
    fn get_next_prime(&mut self) -> i64 {
        while !self.try_next_prime() {};
        *self.primes.last().expect("try_next_prime filled it")
    }
    fn get_prime(&self, pos: usize) -> Option<i64> {
        if pos == 0 {
            Some(2)
        } else if pos-1 < self.primes.len() {
            Some(self.primes[pos-1])
        } else {
            None
        }
    }
}

struct Primes<'a> {
    sieve: &'a mut Sieve,
    pos: usize,
}

impl<'a> Iterator for Primes<'a> {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        Some(self.sieve.get_prime(self.pos - 1).unwrap_or_else(|| self.sieve.get_next_prime()))
    }
}

fn primes(sieve: &mut Sieve) ->  Primes {
    Primes {sieve: sieve, pos: 0}
}

fn test(sieve: &mut Sieve, n : i64) -> Option<i64> {
    let mut v = n;
    let mut primes = primes(sieve);
    let mut max_prime: i64 = 1;
    while let Some(p) = primes.next() {
        while v % p == 0 {
            v = v / p;
            max_prime = p;
        }
        if v == 1 {
            return Some(max_prime);
        }
        if p*p > v {
            return Some(v);
        }
    }
    None
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut sieve = Sieve::default();
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let input_t : i32 = s_t.trim().parse()?;
    for _ in 0..input_t {
        let mut s_n = String::new();
        io::stdin().read_line(&mut s_n)?;
        let n : i64 = s_n.trim().parse()?;
        writeln!(io::stdout(), "{}", test(&mut sieve, n).expect("Unexpected - no more primes"))?;
    }
    Ok(())
}

