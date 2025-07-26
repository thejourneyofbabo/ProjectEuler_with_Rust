//use module_test::{module_abc, module_def};
use module_test::{def_tester, module_abc};

fn main() {
    println!("Hello, world!");
    module_abc::module_abc_test::module_abc_tester();
    def_tester::def_print();
}
