use std::collections::HashMap;

fn main() {
    let divisors = find_divisors(1, 1000000);

    println!("{:?}", divisors);
}

fn triangle_number(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn find_prime_factors(n: i32) -> Vec<i32> {

}

// Finding divisors via trial division with caching
fn find_divisors(min: i32, max: i32) -> HashMap<i32, Vec<i32>> {
    let mut divisor_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for n in min..max {
        let mut divisors: Vec<i32> = vec![];

            for i in 1..n {
                match divisor_map.get(&i) {
                    Some(d) => { divisors.extend(d) },
                    None => { if n % i == 0 { divisors.push(i) } }
                };
            }

        divisors.sort();
        divisors.dedup();

        divisor_map.insert(n, divisors);
    }

    divisor_map
}

