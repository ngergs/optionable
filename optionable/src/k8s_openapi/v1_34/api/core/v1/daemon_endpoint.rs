pub struct DaemonEndpointOpt {
    pub port: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DaemonEndpoint {
    type Optioned = DaemonEndpointOpt;
}
#[automatically_derived]
impl crate::Optionable for DaemonEndpointOpt {
    type Optioned = DaemonEndpointOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DaemonEndpoint {
    fn into_optioned(self) -> DaemonEndpointOpt {
        DaemonEndpointOpt {
            port: Some(self.port),
        }
    }
    fn try_from_optioned(
        value: DaemonEndpointOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: DaemonEndpointOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        Ok(())
    }
}
