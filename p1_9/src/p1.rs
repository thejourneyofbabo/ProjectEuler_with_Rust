// p1.rs
/*
 If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 The sum of these multiples is 23.
 Find the sum of all the multiples of 3 or 5 below N.
*/

#[cfg(test)]
fn get_sum(bound: u64) -> u64 {
    let mut sum = 0;

    for i in 3..bound {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("bound: {}, sum: {}", bound, sum);
    sum
}

#[cfg(test)]
pub mod attempt2 {
    pub fn get_sum2(bound: u64) -> u64 {
        let sum = sum_under2(3, bound - 1) + sum_under2(5, bound - 1) - sum_under2(15, bound - 1);

        println!("bound: {}, sum: {}", bound, sum);

        sum
    }

    fn sum_under2(k: u64, n: u64) -> u64 {
        let p = n / k;
        k * p * (p + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(23, get_sum(10));
        assert_eq!(233168, get_sum(1000));
    }

    #[test]
    fn test2() {
        assert_eq!(23, attempt2::get_sum2(10));
        assert_eq!(233168, attempt2::get_sum2(1000));
        assert_eq!(233333333166666668, attempt2::get_sum2(1000000000));
    }
}
