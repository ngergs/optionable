#[automatically_derived]
struct TestStructOpt {
    street: Option<String>,
    number: Option<i32>,
}
#[automatically_derived]
impl ::optionable::Optionable for TestStruct {
    type Optioned = TestStructOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestStructOpt {
    type Optioned = TestStructOpt;
}


#[automatically_derived]
enum TestEnumOpt {
    Number(Option<i32>),
    Address(Option<<TestStruct as ::optionable::Optionable>::Optioned>),
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnum {
    type Optioned = TestEnumOpt;
}
#[automatically_derived]
impl ::optionable::Optionable for TestEnumOpt {
    type Optioned = TestEnumOpt;
}
