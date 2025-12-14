#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldsV1Ac(
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1 {
    fn into_optioned(self) -> FieldsV1Ac {
        FieldsV1Ac(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: FieldsV1Ac) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: FieldsV1Ac) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1>
for FieldsV1Ac {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldsV1,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
