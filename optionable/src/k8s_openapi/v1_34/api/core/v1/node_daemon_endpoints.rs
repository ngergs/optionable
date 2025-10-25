pub struct NodeDaemonEndpointsOpt {
    pub kubelet_endpoint: <Option<
        ::k8s_openapi::api::core::v1::DaemonEndpoint,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeDaemonEndpoints {
    type Optioned = NodeDaemonEndpointsOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeDaemonEndpointsOpt {
    type Optioned = NodeDaemonEndpointsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeDaemonEndpoints {
    fn into_optioned(self) -> NodeDaemonEndpointsOpt {
        NodeDaemonEndpointsOpt {
            kubelet_endpoint: <Option<
                ::k8s_openapi::api::core::v1::DaemonEndpoint,
            > as crate::OptionableConvert>::into_optioned(self.kubelet_endpoint),
        }
    }
    fn try_from_optioned(
        value: NodeDaemonEndpointsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            kubelet_endpoint: <Option<
                ::k8s_openapi::api::core::v1::DaemonEndpoint,
            > as crate::OptionableConvert>::try_from_optioned(value.kubelet_endpoint)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeDaemonEndpointsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::DaemonEndpoint,
        > as crate::OptionableConvert>::merge(
            &mut self.kubelet_endpoint,
            other.kubelet_endpoint,
        )?;
        Ok(())
    }
}
