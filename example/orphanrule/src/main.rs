// Example how circumvent the orphanrule for some situations

// We use `serde_json::Value` as an example. Note that `optionable` actually provides
// a self resolving implementation for this value with the `serde_json` feature which has
// been disabled for this example though.

use optionable::{Error, Optionable, OptionableConvert, OptionedConvert};
use serde_json::{json, Value};

#[derive(Optionable, PartialEq, Debug, Clone)]
#[optionable(derive(PartialEq, Debug, Clone))]
struct Example {
    #[optionable(optioned_type=JsonValue)]
    value: Value,
}

#[derive(PartialEq, Debug, Clone)]
struct JsonValue(Value);

impl OptionedConvert<Value> for JsonValue {
    fn from_optionable(value: Value) -> Self {
        Self(value)
    }

    fn try_into_optionable(self) -> Result<Value, Error> {
        Ok(self.0)
    }

    fn merge_into(self, other: &mut Value) -> Result<(), Error> {
        *other = self.0;
        Ok(())
    }
}

fn main() {
    let val = Example { value: json!(42) };
    let val_opt = ExampleOpt {
        value: Some(JsonValue(json!(42))),
    };
    assert_eq!(
        val,
        OptionableConvert::try_from_optioned(val_opt.clone()).unwrap()
    );
    assert_eq!(val.clone().into_optioned(), val_opt);
}
