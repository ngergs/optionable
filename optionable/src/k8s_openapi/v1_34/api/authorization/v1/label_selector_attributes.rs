pub struct LabelSelectorAttributesAc {
    pub raw_selector: <Option<std::string::String> as crate::Optionable>::Optioned,
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
        other: LabelSelectorAttributesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.raw_selector, other.raw_selector)?;
        crate::OptionableConvert::merge(&mut self.requirements, other.requirements)?;
        Ok(())
    }
}
