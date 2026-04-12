#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A label selector is a label query over a set of resources. The result of matchLabels and matchExpressions are ANDed. An empty label selector matches all objects. A null label selector matches no objects.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LabelSelectorAc {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_expressions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelectorRequirement as crate::Optionable>::Optioned,
        >,
    >,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_labels: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector {
    type Optioned = LabelSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for LabelSelectorAc {
    type Optioned = LabelSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector {
    fn into_optioned(self) -> LabelSelectorAc {
        LabelSelectorAc {
            match_expressions: crate::OptionableConvert::into_optioned(
                self.match_expressions,
            ),
            match_labels: self.match_labels,
        }
    }
    fn try_from_optioned(value: LabelSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            match_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_expressions,
            )?,
            match_labels: value.match_labels,
        })
    }
    fn merge(&mut self, other: LabelSelectorAc) -> Result<(), crate::Error> {
        if self.match_expressions.is_none() {
            self.match_expressions = crate::OptionableConvert::try_from_optioned(
                other.match_expressions,
            )?;
        } else if let Some(self_value) = self.match_expressions.as_mut()
            && let Some(other_value) = other.match_expressions
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.match_labels.is_none() {
            self.match_labels = crate::OptionableConvert::try_from_optioned(
                other.match_labels,
            )?;
        } else if let Some(self_value) = self.match_labels.as_mut()
            && let Some(other_value) = other.match_labels
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
> for LabelSelectorAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
