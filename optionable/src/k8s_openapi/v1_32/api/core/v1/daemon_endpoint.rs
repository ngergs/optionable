#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DaemonEndpoint {
    fn into_optioned(self) -> DaemonEndpointAc {
        DaemonEndpointAc {
            port: Some(self.port),
        }
    }
    fn try_from_optioned(value: DaemonEndpointAc) -> Result<Self, crate::Error> {
        Ok(Self {
            port: value
                .port
                .ok_or(crate::Error {
                    missing_field: "port",
                })?,
        })
    }
    fn merge(&mut self, other: DaemonEndpointAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::DaemonEndpoint>
for DaemonEndpointAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::DaemonEndpoint) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::DaemonEndpoint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::DaemonEndpoint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
