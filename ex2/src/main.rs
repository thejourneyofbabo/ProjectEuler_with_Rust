//mod sample;
use ex2::{module_test, sample};

fn main() {
    println!("Hello, world!");
    let k = sample::get_sum(10);
    //let j = module_test::try_mod();
    module_test::try_mod();
    println!("{k}");
}
