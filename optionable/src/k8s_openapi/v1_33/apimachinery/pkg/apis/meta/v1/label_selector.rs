#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LabelSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_expressions: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
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
    fn try_from_optioned(value: LabelSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            match_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_expressions,
            )?,
            match_labels: crate::OptionableConvert::try_from_optioned(
                value.match_labels,
            )?,
        })
    }
    fn merge(&mut self, other: LabelSelectorAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_expressions,
            other.match_expressions,
        )?;
        crate::OptionableConvert::merge(&mut self.match_labels, other.match_labels)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
> for LabelSelectorAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
