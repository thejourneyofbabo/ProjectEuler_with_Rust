// p7.rs
/*
By listing the first six prime numbers: 2, 3, 5, 7, 11 and 13, we can see that the 6th prime is 13. What is the 10001st prime number?
*/

#[cfg(test)]
fn base() {
    println!("base");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("test_template");
        base();
    }
}
