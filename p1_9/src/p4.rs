// p4.rs
/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 X 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

#[allow(dead_code)]
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

#[allow(dead_code)]
#[cfg(test)]
fn find_palindrome() -> u32 {
    // 1. find from [999..=100]
    let mut a: u32 = 999;
    let mut max: u32 = 1;
    while a >= 100 {
        let mut b: u32 = 999;
        let mut p: u32;
        while b >= a {
            p = a * b;
            if p <= max {
                break;
            }
            if is_palindrome(p) && p > max {
                max = p;
            }
            b -= 1;
        }
        a -= 1;
    }
    max
}

#[cfg(test)]
// check the palindrome number of n is the product of two integer of not
fn is_product(n: u32) -> bool {
    let mut ans: bool = false;
    for j in (100..=999).rev() {
        let (q, r): (u32, u32) = (n / j, n % j);
        if r == 0 && q <= 999 {
            ans = true;
            break;
        }
        if q > 999 {
            break;
        }
    }
    ans
}

#[cfg(test)]
fn rev_num(mut n: u32) -> u32 {
    let mut rev: u32 = 0;
    while n > 0 {
        rev = 10 * rev + n % 10;
        n /= 10;
    }
    rev
}

#[cfg(test)]
fn find_palindrome3() -> u32 {
    // 1. make plaindromic num
    let mut x: u32 = 999;
    let mut max: u32 = 1;
    while x >= 100 {
        let y = rev_num(x);
        let p = x * 1000 + y;
        if is_product(p) {
            max = p;
            break;
        }
        x -= 1;
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", find_palindrome3());
    }
}
