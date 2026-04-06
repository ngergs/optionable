#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// StatefulSetOrdinals describes the policy used for replica ordinal assignment in this StatefulSet.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetOrdinalsAc {
    /// start is the number representing the first replica's index. It may be used to number replicas from an alternate index (eg: 1-indexed) over the default 0-indexed names, or to orchestrate progressive movement of replicas from one StatefulSet to another. If set, replica indices will be in the range:
    ///   \[.spec.ordinals.start, .spec.ordinals.start + .spec.replicas).
    /// If unset, defaults to 0. Replica indices will be in the range:
    ///   \[0, .spec.replicas).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::StatefulSetOrdinals {
    type Optioned = StatefulSetOrdinalsAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetOrdinalsAc {
    type Optioned = StatefulSetOrdinalsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::StatefulSetOrdinals {
    fn into_optioned(self) -> StatefulSetOrdinalsAc {
        StatefulSetOrdinalsAc {
            start: self.start,
        }
    }
    fn try_from_optioned(value: StatefulSetOrdinalsAc) -> Result<Self, crate::Error> {
        Ok(Self { start: value.start })
    }
    fn merge(&mut self, other: StatefulSetOrdinalsAc) -> Result<(), crate::Error> {
        self.start = other.start;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::StatefulSetOrdinals>
for StatefulSetOrdinalsAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::StatefulSetOrdinals,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::StatefulSetOrdinals, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetOrdinals,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
