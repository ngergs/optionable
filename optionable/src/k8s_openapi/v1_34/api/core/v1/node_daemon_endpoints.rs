pub struct NodeDaemonEndpointsAc {
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
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeDaemonEndpoints {
    fn into_optioned(self) -> NodeDaemonEndpointsAc {
        NodeDaemonEndpointsAc {
            kubelet_endpoint: crate::OptionableConvert::into_optioned(
                self.kubelet_endpoint,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeDaemonEndpointsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            kubelet_endpoint: crate::OptionableConvert::try_from_optioned(
                value.kubelet_endpoint,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeDaemonEndpointsAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.kubelet_endpoint,
            other.kubelet_endpoint,
        )?;
        Ok(())
    }
}
