pub fn nth(n: u32) -> u32 {
    primes().nth(n as usize).unwrap()
}

pub struct Primes {
    primes: Vec<u32>,
}

pub fn primes() -> Primes {
    Primes { primes: Vec::new() }
}

impl Iterator for Primes {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut candidate = self.primes.last().copied().unwrap_or(1);
        loop {
            candidate += 1 + (candidate > 2) as Self::Item;
            if self.primes.iter()
                .take_while(|&p| p * p <= candidate)
                .all(|&p| candidate % p != 0)
            {
                self.primes.push(candidate);
                return Some(candidate);
            }
        }
    }
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}