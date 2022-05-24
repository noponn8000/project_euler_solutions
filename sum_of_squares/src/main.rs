fn main() {
    let sum_of_squares: i64 = sum_of_squares(100);
    let square: i64 = square_of_sum(100);

    let difference: i64 = square - sum_of_squares;

    println!("Sum of squares: {}", sum_of_squares);
    println!("Square of sums: {}", square);
    println!("The difference is {}", difference);
}

fn sum_of_squares(n: i64) -> i64 {
    let mut sum: i64 = 0;

    for i in 1..=n {
        sum += i * i;
    }

    return sum;
}

fn square_of_sum(n: i64) -> i64 {
    let mut sum: i64 = 0;
    // formula: -> -> -> n * (n + 1) / 2;

    for i in 1..=n {
        sum += i;
    }

    return sum * sum;
}
