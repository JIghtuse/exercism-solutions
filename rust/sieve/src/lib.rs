pub struct Sieve;

impl Sieve {
    pub fn primes_up_to(n: u32) -> Vec<u32> {
        let mut result = (2 .. n + 1).collect::<Vec<u32>>();
        let mut pos = 0;
        while pos != result.len() {
            let prime = result[pos];
            result.retain(|&i| i == prime || i % prime != 0);
            pos += 1;
        }
        result
    }
}
