pub struct LabelSelectorAc {
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
    type Optioned = LabelSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for LabelSelectorAc {
    type Optioned = LabelSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
    fn into_optioned(self) -> LabelSelectorAc {
        LabelSelectorAc {
            match_expressions: crate::OptionableConvert::into_optioned(
                self.match_expressions,
            ),
            match_labels: crate::OptionableConvert::into_optioned(self.match_labels),
        }
    }
    fn try_from_optioned(
        value: LabelSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            match_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_expressions,
            )?,
            match_labels: crate::OptionableConvert::try_from_optioned(
                value.match_labels,
            )?,
        })
    }
    fn merge(&mut self, other: LabelSelectorAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_expressions,
            other.match_expressions,
        )?;
        crate::OptionableConvert::merge(&mut self.match_labels, other.match_labels)?;
        Ok(())
    }
}
