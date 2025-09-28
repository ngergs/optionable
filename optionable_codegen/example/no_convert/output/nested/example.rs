#[automatically_derived]
struct TestStructNestedOpt {
    street_nested: Option<String>,
    number_nested: Option<i32>,
}
#[automatically_derived]
impl ::optionable::Optionable for TestStructNested {
    type Optioned = TestStructNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestStructNestedOpt {
    type Optioned = TestStructNestedOpt;
}


#[automatically_derived]
enum TestEnumNestedOpt {
    NumberNested(Option<i32>),
    AddressNested(Option<<TestStructNested as ::optionable::Optionable>::Optioned>),
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnumNested {
    type Optioned = TestEnumNestedOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnumNestedOpt {
    type Optioned = TestEnumNestedOpt;
}
