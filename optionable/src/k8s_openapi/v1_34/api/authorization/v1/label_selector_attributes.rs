pub struct LabelSelectorAttributesOpt {
    pub raw_selector: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub requirements: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::label_selector_attributes::LabelSelectorAttributes {
    type Optioned = LabelSelectorAttributesOpt;
}
#[automatically_derived]
impl crate::Optionable for LabelSelectorAttributesOpt {
    type Optioned = LabelSelectorAttributesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::label_selector_attributes::LabelSelectorAttributes {
    fn into_optioned(self) -> LabelSelectorAttributesOpt {
        LabelSelectorAttributesOpt {
            raw_selector: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.raw_selector),
            requirements: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
                >,
            > as crate::OptionableConvert>::into_optioned(self.requirements),
        }
    }
    fn try_from_optioned(
        value: LabelSelectorAttributesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            raw_selector: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.raw_selector)?,
            requirements: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.requirements)?,
        })
    }
    fn merge(
        &mut self,
        other: LabelSelectorAttributesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.raw_selector,
            other.raw_selector,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.requirements,
            other.requirements,
        )?;
        Ok(())
    }
}
