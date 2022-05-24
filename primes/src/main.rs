fn main() {
    let mut primes_found = 0;
    let mut candidate = 2;
    let mut primes: Vec<i32> = Vec::new();

    while (primes_found < 10001) {
        if is_prime(candidate) {
            primes.push(candidate);
            primes_found += 1;
            println!("Prime found! {}", candidate);
        }

        candidate += 1
    }
}

fn is_prime(n: i32) -> bool {
    let mut d = 2;

    while d * d <= n {
        if n % d == 0 {
            return false;
        }

        d += 1;
    }

    true
}
