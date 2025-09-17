// p6.rs
/*
The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 + ... 10^2 = 385

The sauare of the sum of the first ten natural numbers is,
(1 + 2 + ... 10)^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640

Find the difference between the sum of squares of the first one hundred natural numbers and the square of the sum.
*/

#[cfg(test)]
fn get_seq_sum(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[allow(dead_code)]
#[cfg(test)]
fn get_square_sum(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        sum += i * i;
    }
    sum
}

#[cfg(test)]
fn get_square_sum1(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

#[cfg(test)]
fn answer(n: u64) -> u64 {
    // 1. a = (1 + .. + 100)^2
    let mut a: u64 = get_seq_sum(n);
    a *= a;

    // 2. b = (1^2 + ... 100^2)
    let b: u64 = get_square_sum1(n);

    // 3. a-b
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(0, answer(1));
        assert_eq!(4, answer(2));
        assert_eq!(22, answer(3));
        assert_eq!(2640, answer(10));
        println!("{}", answer(100));
    }
}
