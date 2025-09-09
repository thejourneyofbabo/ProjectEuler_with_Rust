// p3.rs
/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?
*/

#[cfg(test)]
fn find_answer(mut n: u64) -> u64 {
    let mut p = 2; // 2 smallest prime number
    loop {
        let (q, r) = (n / p, n % p); // use tuple

        if q == 1 {
            break;
        }

        if r == 0 {
            n = q;
        }
        // if else
        else {
            p += 1;
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("test_template");
        assert_eq!(5, find_answer(10));
        assert_eq!(2, find_answer(16));
        assert_eq!(17, find_answer(17));

        assert_eq!(29, find_answer(13195));
        println!("{}", find_answer(600851475143));
    }
}
