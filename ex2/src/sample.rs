// sample.rs
//

#[test]
pub fn test() {
    println!("1+...+100 = {}", get_sum(100));
    assert_eq!(5050, get_sum(100));
}

//#[cfg(test)]
pub fn get_sum(n: u32) -> u32 {
    let mut sum: u32 = 0;

    for i in 1..=n {
        sum += i;
    }

    sum
}
