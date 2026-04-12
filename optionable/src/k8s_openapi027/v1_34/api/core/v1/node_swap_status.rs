#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeSwapStatus represents swap memory information.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSwapStatusAc {
    /// Total amount of swap memory in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeSwapStatus {
    type Optioned = NodeSwapStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSwapStatusAc {
    type Optioned = NodeSwapStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeSwapStatus {
    fn into_optioned(self) -> NodeSwapStatusAc {
        NodeSwapStatusAc {
            capacity: self.capacity,
        }
    }
    fn try_from_optioned(value: NodeSwapStatusAc) -> Result<Self, crate::Error> {
        Ok(Self { capacity: value.capacity })
    }
    fn merge(&mut self, other: NodeSwapStatusAc) -> Result<(), crate::Error> {
        if self.capacity.is_none() {
            self.capacity = crate::OptionableConvert::try_from_optioned(other.capacity)?;
        } else if let Some(self_value) = self.capacity.as_mut()
            && let Some(other_value) = other.capacity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeSwapStatus>
for NodeSwapStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::NodeSwapStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeSwapStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeSwapStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
