// p5.rs
/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evrnly divisible by all of the numbers from 1 to 20?
*/

#[cfg(test)]
fn gcd(mut a: u64, mut b: u64) -> u64 {
    // 1) could update parametercd
    let mut t: u64;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[allow(dead_code)]
#[cfg(test)]
fn answer(n: u64) -> u64 {
    let mut ans: u64 = 1;
    for i in 2..=n {
        ans = lcm(ans, i);
    }
    ans
}

#[cfg(test)]
fn answer1(n: u64) -> u64 {
    let mut ans: u64 = 1;
    for i in 2..=n {
        if ans % i > 0 {
            ans = lcm(ans, i);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, answer1(1));
        assert_eq!(2, answer1(2));
        assert_eq!(6, answer1(3));
        assert_eq!(2520, answer1(10));

        println!("LCM 1 to 20 = {}", answer1(20));
    }
}
