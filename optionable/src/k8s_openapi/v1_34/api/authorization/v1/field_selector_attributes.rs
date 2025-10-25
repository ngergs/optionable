pub struct FieldSelectorAttributesOpt {
    pub raw_selector: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub requirements: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes {
    type Optioned = FieldSelectorAttributesOpt;
}
#[automatically_derived]
impl crate::Optionable for FieldSelectorAttributesOpt {
    type Optioned = FieldSelectorAttributesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::FieldSelectorAttributes {
    fn into_optioned(self) -> FieldSelectorAttributesOpt {
        FieldSelectorAttributesOpt {
            raw_selector: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.raw_selector),
            requirements: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
                >,
            > as crate::OptionableConvert>::into_optioned(self.requirements),
        }
    }
    fn try_from_optioned(
        value: FieldSelectorAttributesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            raw_selector: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.raw_selector)?,
            requirements: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.requirements)?,
        })
    }
    fn merge(
        &mut self,
        other: FieldSelectorAttributesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.raw_selector,
            other.raw_selector,
        )?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.requirements,
            other.requirements,
        )?;
        Ok(())
    }
}
