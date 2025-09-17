mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod test_template;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
