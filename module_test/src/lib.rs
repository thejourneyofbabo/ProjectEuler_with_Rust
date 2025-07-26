// Module ABC
pub mod module_abc {
    pub mod module_abc_test;
}

// Module DEF
pub mod module_def {
    pub mod def_tester;
}

pub use module_def::def_tester;
