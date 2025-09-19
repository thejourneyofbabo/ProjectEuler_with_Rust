// p7.rs
/*
By listing the first six prime numbers: 2, 3, 5, 7, 11 and 13, we can see that the 6th prime is 13. What is the 10001st prime number?
*/

#[cfg(test)]
fn answer(n: u64) -> u64 {
    let mut ans: u64 = 2;
    let mut cnt: u64 = 1;
    while cnt < n {
        ans += 1;
        let kmax: u64 = (ans as f64).sqrt() as u64;
        let mut flag = true;
        for k in 2..=kmax {
            if ans % k == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            cnt += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, answer(1));
        assert_eq!(3, answer(2));
        assert_eq!(5, answer(3));
        assert_eq!(13, answer(6));
        println!("{}", answer(10001));
    }
}
