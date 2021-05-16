pub trait Operator<T> {
    fn operate(&self, operand: &T) -> T;
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct TestStruct<T> {
    pub x: T,
}

impl Operator<TestStruct<u32>> for TestStruct<u32> {
    fn operate(&self, operand: &TestStruct<u32>) -> TestStruct<u32> {
        TestStruct {
            x: (self.x + operand.x) % 12,
        }
    }
}

impl Operator<TestStruct<i32>> for TestStruct<i32> {
    fn operate(&self, operand: &TestStruct<i32>) -> TestStruct<i32> {
        TestStruct {
            x: (self.x - operand.x) % 12,
        }
    }
}
