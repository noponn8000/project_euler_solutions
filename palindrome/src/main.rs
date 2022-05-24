fn main() {
    let mut product: i32 = 0;
    let mut palindromes: Vec<String> = Vec::new();

    for i in 100..999 {
        for j in 100..999 {
            product = i * j;

            let product_string: String = product.to_string();
            let product_string_reversed: String = product.to_string()
                .chars().rev().collect();

            if product_string == product_string_reversed {
                palindromes.push(product_string);
            }
        }
    }

    let mut palindrome_integers: Vec<i32> = Vec::new();

    for i in palindromes {
        palindrome_integers.push(i.parse::<i32>().unwrap());
    }

    palindrome_integers.sort();

    println!("The largest palindromic product of 2 3-digits integers is {}.",
             palindrome_integers.last().unwrap());
}
