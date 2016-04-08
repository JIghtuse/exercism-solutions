pub struct Sieve;

impl Sieve {
    pub fn primes_up_to(n: u32) -> Vec<u32> {
        (2 .. n + 1).collect()
    }
}
