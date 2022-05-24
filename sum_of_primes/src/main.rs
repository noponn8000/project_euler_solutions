fn main() {
    let mut sum: i64 = 0;

    for i in 2..2000000 {
        if is_prime(i) {
            sum += i;
        }
    }

    println!("The sum of all primes below 2000000 is {}.", sum);
}

fn is_prime(n: i64) -> bool {
    let mut d = 2;

    while d * d <= n {
        if n % d == 0 {
            return false;
        }

        d += 1;
    }

    true
}
