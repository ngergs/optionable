#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeDaemonEndpoints lists ports opened by daemons running on the Node.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeDaemonEndpointsAc {
    /// Endpoint on which Kubelet is listening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_endpoint: Option<
        <::k8s_openapi027::api::core::v1::DaemonEndpoint as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeDaemonEndpoints {
    type Optioned = NodeDaemonEndpointsAc;
}
#[automatically_derived]
impl crate::Optionable for NodeDaemonEndpointsAc {
    type Optioned = NodeDaemonEndpointsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::NodeDaemonEndpoints {
    fn into_optioned(self) -> NodeDaemonEndpointsAc {
        NodeDaemonEndpointsAc {
            kubelet_endpoint: crate::OptionableConvert::into_optioned(
                self.kubelet_endpoint,
            ),
        }
    }
    fn try_from_optioned(value: NodeDaemonEndpointsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            kubelet_endpoint: crate::OptionableConvert::try_from_optioned(
                value.kubelet_endpoint,
            )?,
        })
    }
    fn merge(&mut self, other: NodeDaemonEndpointsAc) -> Result<(), crate::Error> {
        if self.kubelet_endpoint.is_none() {
            self.kubelet_endpoint = crate::OptionableConvert::try_from_optioned(
                other.kubelet_endpoint,
            )?;
        } else if let Some(self_value) = self.kubelet_endpoint.as_mut()
            && let Some(other_value) = other.kubelet_endpoint
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeDaemonEndpoints>
for NodeDaemonEndpointsAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::NodeDaemonEndpoints,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::NodeDaemonEndpoints, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeDaemonEndpoints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
