use crate::{OptionableConvert, OptionedConvert};
use fake::{Dummy, Fake, Faker};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

#[allow(unused)]
pub(crate) fn roundtrip_test<T>()
where
    T: Debug + Clone + PartialEq + Dummy<Faker> + Serialize + DeserializeOwned + OptionableConvert,
    T::Optioned: Debug + Clone + PartialEq + Serialize + DeserializeOwned + OptionedConvert<T>,
{
    let val: T = Faker.fake();
    let val_json = serde_json::to_value(val.clone()).unwrap();
    let val: T = serde_json::from_value(val_json.clone()).unwrap();
    let val_ac: T::Optioned = serde_json::from_value(val_json).unwrap();
    assert_eq!(val.clone().into_optioned(), val_ac);
    assert_eq!(val, val_ac.try_into_optionable().unwrap());
}
