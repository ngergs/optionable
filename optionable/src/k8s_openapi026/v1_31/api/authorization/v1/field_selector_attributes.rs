#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldSelectorAttributesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_selector: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: <Option<
        std::vec::Vec<
            ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::authorization::v1::FieldSelectorAttributes {
    type Optioned = FieldSelectorAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for FieldSelectorAttributesAc {
    type Optioned = FieldSelectorAttributesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::authorization::v1::FieldSelectorAttributes {
    fn into_optioned(self) -> FieldSelectorAttributesAc {
        FieldSelectorAttributesAc {
            raw_selector: crate::OptionableConvert::into_optioned(self.raw_selector),
            requirements: crate::OptionableConvert::into_optioned(self.requirements),
        }
    }
    fn try_from_optioned(
        value: FieldSelectorAttributesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            raw_selector: crate::OptionableConvert::try_from_optioned(
                value.raw_selector,
            )?,
            requirements: crate::OptionableConvert::try_from_optioned(
                value.requirements,
            )?,
        })
    }
    fn merge(&mut self, other: FieldSelectorAttributesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.raw_selector, other.raw_selector)?;
        crate::OptionableConvert::merge(&mut self.requirements, other.requirements)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::authorization::v1::FieldSelectorAttributes,
> for FieldSelectorAttributesAc {
    fn from_optionable(
        value: k8s_openapi026::api::authorization::v1::FieldSelectorAttributes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::authorization::v1::FieldSelectorAttributes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::authorization::v1::FieldSelectorAttributes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
