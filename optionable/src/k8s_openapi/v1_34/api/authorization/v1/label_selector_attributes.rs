#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct LabelSelectorAttributesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_selector: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::LabelSelectorAttributes {
    type Optioned = LabelSelectorAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for LabelSelectorAttributesAc {
    type Optioned = LabelSelectorAttributesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::LabelSelectorAttributes {
    fn into_optioned(self) -> LabelSelectorAttributesAc {
        LabelSelectorAttributesAc {
            raw_selector: crate::OptionableConvert::into_optioned(self.raw_selector),
            requirements: crate::OptionableConvert::into_optioned(self.requirements),
        }
    }
    fn try_from_optioned(
        value: LabelSelectorAttributesAc,
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
    fn merge(&mut self, other: LabelSelectorAttributesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.raw_selector, other.raw_selector)?;
        crate::OptionableConvert::merge(&mut self.requirements, other.requirements)?;
        Ok(())
    }
}
