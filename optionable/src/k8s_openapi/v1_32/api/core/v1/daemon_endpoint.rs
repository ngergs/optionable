#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DaemonEndpointAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DaemonEndpoint {
    type Optioned = DaemonEndpointAc;
}
#[automatically_derived]
impl crate::Optionable for DaemonEndpointAc {
    type Optioned = DaemonEndpointAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DaemonEndpoint {
    fn into_optioned(self) -> DaemonEndpointAc {
        DaemonEndpointAc {
            port: Some(self.port),
        }
    }
    fn try_from_optioned(
        value: DaemonEndpointAc,
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
        other: DaemonEndpointAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        Ok(())
    }
}
