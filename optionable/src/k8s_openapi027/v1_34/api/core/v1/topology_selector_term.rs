#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// A topology selector term represents the result of label queries. A null or empty topology selector term matches no objects. The requirements of them are ANDed. It provides a subset of functionality as NodeSelectorTerm. This is an alpha feature and may change in the future.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TopologySelectorTermAc {
    /// A list of topology selector requirements by labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_label_expressions: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::TopologySelectorLabelRequirement as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::TopologySelectorTerm {
    type Optioned = TopologySelectorTermAc;
}
#[automatically_derived]
impl crate::Optionable for TopologySelectorTermAc {
    type Optioned = TopologySelectorTermAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::TopologySelectorTerm {
    fn into_optioned(self) -> TopologySelectorTermAc {
        TopologySelectorTermAc {
            match_label_expressions: crate::OptionableConvert::into_optioned(
                self.match_label_expressions,
            ),
        }
    }
    fn try_from_optioned(value: TopologySelectorTermAc) -> Result<Self, crate::Error> {
        Ok(Self {
            match_label_expressions: crate::OptionableConvert::try_from_optioned(
                value.match_label_expressions,
            )?,
        })
    }
    fn merge(&mut self, other: TopologySelectorTermAc) -> Result<(), crate::Error> {
        if self.match_label_expressions.is_none() {
            self.match_label_expressions = crate::OptionableConvert::try_from_optioned(
                other.match_label_expressions,
            )?;
        } else if let Some(self_value) = self.match_label_expressions.as_mut()
            && let Some(other_value) = other.match_label_expressions
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::TopologySelectorTerm>
for TopologySelectorTermAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::TopologySelectorTerm,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::TopologySelectorTerm, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::TopologySelectorTerm,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for TopologySelectorTermAc {
    fn merge_from(&mut self, other: Self) {
        self.match_label_expressions = other.match_label_expressions;
    }
}
