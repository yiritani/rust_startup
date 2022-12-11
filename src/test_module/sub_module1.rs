mod sub_module1 {
    pub fn sub1_func() {
        println!("sub1_func");
    }
    fn sub1_func2() {
        println!("sub1_func2");
    }
}

pub(crate) struct TestStruct {
    val1: i32,
    val2: i32,
}

impl TestStruct {
    pub fn new(val1: i32, val2: i32) -> TestStruct {
        TestStruct { val1, val2 }
    }
}
