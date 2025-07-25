// p2.rs
/*
Problems
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
