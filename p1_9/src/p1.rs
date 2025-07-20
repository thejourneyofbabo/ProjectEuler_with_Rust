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
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(23, get_sum(10));
        assert_eq!(233168, get_sum(1000));
    }
}
