#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSelectorAttributesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_selector: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes {
    type Optioned = FieldSelectorAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for FieldSelectorAttributesAc {
    type Optioned = FieldSelectorAttributesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes {
    fn into_optioned(self) -> FieldSelectorAttributesAc {
        FieldSelectorAttributesAc {
            raw_selector: crate::OptionableConvert::into_optioned(self.raw_selector),
            requirements: crate::OptionableConvert::into_optioned(self.requirements),
        }
    }
    fn try_from_optioned(
        value: FieldSelectorAttributesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            raw_selector: crate::OptionableConvert::try_from_optioned(
                value.raw_selector,
            )?,
            requirements: crate::OptionableConvert::try_from_optioned(
                value.requirements,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: FieldSelectorAttributesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.raw_selector, other.raw_selector)?;
        crate::OptionableConvert::merge(&mut self.requirements, other.requirements)?;
        Ok(())
    }
}
