#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NodeDaemonEndpointsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_endpoint: <Option<
        ::k8s_openapi::api::core::v1::DaemonEndpoint,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeDaemonEndpoints {
    type Optioned = NodeDaemonEndpointsAc;
}
#[automatically_derived]
impl crate::Optionable for NodeDaemonEndpointsAc {
    type Optioned = NodeDaemonEndpointsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeDaemonEndpoints {
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
        crate::OptionableConvert::merge(
            &mut self.kubelet_endpoint,
            other.kubelet_endpoint,
        )?;
        Ok(())
    }
}
