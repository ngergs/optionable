#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Overhead structure represents the resource overhead associated with running a pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OverheadAc {
    /// podFixed represents the fixed resource overhead associated with running a pod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_fixed: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::node::v1::Overhead {
    type Optioned = OverheadAc;
}
#[automatically_derived]
impl crate::Optionable for OverheadAc {
    type Optioned = OverheadAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::node::v1::Overhead {
    fn into_optioned(self) -> OverheadAc {
        OverheadAc {
            pod_fixed: crate::OptionableConvert::into_optioned(self.pod_fixed),
        }
    }
    fn try_from_optioned(value: OverheadAc) -> Result<Self, crate::Error> {
        Ok(Self {
            pod_fixed: crate::OptionableConvert::try_from_optioned(value.pod_fixed)?,
        })
    }
    fn merge(&mut self, other: OverheadAc) -> Result<(), crate::Error> {
        if self.pod_fixed.is_none() {
            self.pod_fixed = crate::OptionableConvert::try_from_optioned(
                other.pod_fixed,
            )?;
        } else if let Some(self_value) = self.pod_fixed.as_mut()
            && let Some(other_value) = other.pod_fixed
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::node::v1::Overhead> for OverheadAc {
    fn from_optionable(value: k8s_openapi027::api::node::v1::Overhead) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::node::v1::Overhead, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::node::v1::Overhead,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for OverheadAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.pod_fixed, other.pod_fixed);
    }
}
