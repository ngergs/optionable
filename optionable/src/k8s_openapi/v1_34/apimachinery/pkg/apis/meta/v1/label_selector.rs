pub struct LabelSelectorOpt {
    pub match_expressions: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
    pub match_labels: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
    type Optioned = LabelSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for LabelSelectorOpt {
    type Optioned = LabelSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
    fn into_optioned(self) -> LabelSelectorOpt {
        LabelSelectorOpt {
            match_expressions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
                >,
            > as crate::OptionableConvert>::into_optioned(self.match_expressions),
            match_labels: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.match_labels),
        }
    }
    fn try_from_optioned(
        value: LabelSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_expressions: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.match_expressions)?,
            match_labels: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.match_labels)?,
        })
    }
    fn merge(
        &mut self,
        other: LabelSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.match_expressions,
            other.match_expressions,
        )?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.match_labels,
            other.match_labels,
        )?;
        Ok(())
    }
}
