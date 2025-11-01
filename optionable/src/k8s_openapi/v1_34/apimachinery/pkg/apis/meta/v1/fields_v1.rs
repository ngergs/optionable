pub struct FieldsV1Ac(
    pub Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1 {
    type Optioned = FieldsV1Ac;
}
#[automatically_derived]
impl crate::Optionable for FieldsV1Ac {
    type Optioned = FieldsV1Ac;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1 {
    fn into_optioned(self) -> FieldsV1Ac {
        FieldsV1Ac(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: FieldsV1Ac) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: FieldsV1Ac) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
