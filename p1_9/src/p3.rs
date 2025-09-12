// p3.rs
/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?
*/

#[allow(dead_code)]
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

#[allow(dead_code)]
#[cfg(test)]
fn find_answer1(mut n: u64) -> u64 {
    let mut p = 2;
    while n > 1 {
        if 0 == n % p {
            n /= p;
        } else {
            p += 1;
        }
    }
    if n != 1 {
        p = n;
    }
    p
}

#[allow(dead_code)]
#[cfg(test)]
fn find_answer2(mut n: u64) -> u64 {
    let mut p = 2;
    let mut max_factor = (n as f64).sqrt() as u64; // 1) sqrt(n)

    while n > 1 && p <= max_factor {
        if 0 == n % p {
            n /= p;
            max_factor = (n as f64).sqrt() as u64;
        } else {
            p += 1;
        }
    }

    if n != 1 {
        p = n;
    }

    p
}

#[cfg(test)]
fn find_answer3(mut n: u64) -> u64 {
    // Check about 2
    let mut last_factor = 2;
    if n % 2 == 0 {
        n /= 2;
        while n % 2 == 0 {
            n /= 2;
        }
    } else {
        last_factor = 1;
    }

    // Prime number over 2
    let mut p = 3;
    let mut max_factor = (n as f64).sqrt() as u64;
    while n > 1 && p <= max_factor {
        if 0 == n % p {
            n /= p;
            last_factor = p;
            max_factor = (n as f64).sqrt() as u64;
        } else {
            p += 2; // Can check every 2 steps
        }
    }
    if n != 1 {
        last_factor = n;
    }

    last_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("test_template");
        assert_eq!(2, find_answer3(16));
        assert_eq!(5, find_answer3(10));
        assert_eq!(17, find_answer3(17));

        assert_eq!(29, find_answer3(13195));
        println!("{}", find_answer3(600851475143));
    }
}
