#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OverheadAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_fixed: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi027::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
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
        crate::OptionableConvert::merge(&mut self.pod_fixed, other.pod_fixed)?;
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
