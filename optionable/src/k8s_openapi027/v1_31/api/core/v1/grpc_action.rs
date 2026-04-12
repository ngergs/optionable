#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GRPCActionAc {
    /// Port number of the gRPC service. Number must be in the range 1 to 65535.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).
    ///
    /// If this is not specified, the default behavior is defined by gRPC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::GRPCAction {
    type Optioned = GRPCActionAc;
}
#[automatically_derived]
impl crate::Optionable for GRPCActionAc {
    type Optioned = GRPCActionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::GRPCAction {
    fn into_optioned(self) -> GRPCActionAc {
        GRPCActionAc {
            port: Some(self.port),
            service: self.service,
        }
    }
    fn try_from_optioned(value: GRPCActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            port: value
                .port
                .ok_or(crate::Error {
                    missing_field: "port",
                })?,
            service: value.service,
        })
    }
    fn merge(&mut self, other: GRPCActionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        if other.service.is_some() {
            self.service = other.service;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::GRPCAction> for GRPCActionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::GRPCAction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::GRPCAction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::GRPCAction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
