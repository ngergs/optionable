struct TestStructNested {
    street_nested: String,
    number_nested: i32,
}

enum TestEnumNested {
    NumberNested(i32),
    AddressNested(TestStructNested),
}
