#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// GRPCAction specifies an action involving a GRPC service.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GRPCActionAc {
    /// mode specifies the connection mode for the gRPC health probe. Set to "TLS" to use TLS without certificate verification. Set to "Plaintext" to use a plaintext (insecure) connection explicitly. If not specified, the probe uses a plaintext (insecure) connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<std::string::String>,
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
impl crate::Optionable for k8s_openapi029::api::core::v1::GRPCAction {
    type Optioned = GRPCActionAc;
}
#[automatically_derived]
impl crate::Optionable for GRPCActionAc {
    type Optioned = GRPCActionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi029::api::core::v1::GRPCAction {
    fn into_optioned(self) -> GRPCActionAc {
        GRPCActionAc {
            mode: self.mode,
            port: Some(self.port),
            service: self.service,
        }
    }
    fn try_from_optioned(value: GRPCActionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            mode: value.mode,
            port: value
                .port
                .ok_or(crate::Error {
                    missing_field: "port",
                })?,
            service: value.service,
        })
    }
    fn merge(&mut self, other: GRPCActionAc) -> Result<(), crate::Error> {
        if self.mode.is_none() {
            self.mode = crate::OptionableConvert::try_from_optioned(other.mode)?;
        } else if let Some(self_value) = self.mode.as_mut()
            && let Some(other_value) = other.mode
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.port {
            self.port = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.service.is_none() {
            self.service = crate::OptionableConvert::try_from_optioned(other.service)?;
        } else if let Some(self_value) = self.service.as_mut()
            && let Some(other_value) = other.service
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::core::v1::GRPCAction> for GRPCActionAc {
    fn from_optionable(value: k8s_openapi029::api::core::v1::GRPCAction) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::core::v1::GRPCAction, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::core::v1::GRPCAction,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for GRPCActionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.mode, other.mode);
        k8s_openapi029::DeepMerge::merge_from(&mut self.port, other.port);
        k8s_openapi029::DeepMerge::merge_from(&mut self.service, other.service);
    }
}
