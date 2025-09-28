struct TestStruct {
    street: String,
    number: i32,
}

enum TestEnum {
    Number(i32),
    Address(TestStruct),
}
