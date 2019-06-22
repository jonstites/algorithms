extern crate algorithms;

fn main() {
    println!(
        "{:?}",
        algorithms::primes::sieve_of_eratosthenes_odds(500000000_usize)
    );
}
