// p4.rs
/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 X 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

#[cfg(test)]
fn is_palindrome(num: u32) -> bool {
    let mut n = num;
    let mut rev: u32 = 0;
    while n > 0 {
        rev = 10 * rev + n % 10;
        n /= 10;
    }

    num == rev
}

#[cfg(test)]
fn find_palindrome() -> u32 {
    // 1. find from [100..=999]
    let mut a: u32 = 100;
    let mut max: u32 = 1;
    while a <= 999 {
        let mut b: u32 = 100;
        let mut p: u32;
        while b <= 999 {
            p = a * b;
            if is_palindrome(p) && p > max {
                max = p;
            }
            b += 1;
        }
        a += 1;
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", find_palindrome());
    }
}
