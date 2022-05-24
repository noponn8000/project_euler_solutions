fn main() {
    let mut smallest_multiple = 20;

    while !evenly_divisible(smallest_multiple) {
        smallest_multiple += 1;
    }

    println!("The smallest multiple evenly divisible by numbers 1 to 20 is {}",
    smallest_multiple);
}

fn evenly_divisible(n: i32) -> bool {
    for i in 1..20 {
        if n % i != 0 {
            return false;
        }
    }

    return true;
}
