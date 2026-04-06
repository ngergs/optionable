#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProbeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<
        <::k8s_openapi027::api::core::v1::ExecAction as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<
        <::k8s_openapi027::api::core::v1::GRPCAction as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<
        <::k8s_openapi027::api::core::v1::HTTPGetAction as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<
        <::k8s_openapi027::api::core::v1::TCPSocketAction as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Probe {
    type Optioned = ProbeAc;
}
#[automatically_derived]
impl crate::Optionable for ProbeAc {
    type Optioned = ProbeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Probe {
    fn into_optioned(self) -> ProbeAc {
        ProbeAc {
            exec: crate::OptionableConvert::into_optioned(self.exec),
            failure_threshold: self.failure_threshold,
            grpc: crate::OptionableConvert::into_optioned(self.grpc),
            http_get: crate::OptionableConvert::into_optioned(self.http_get),
            initial_delay_seconds: self.initial_delay_seconds,
            period_seconds: self.period_seconds,
            success_threshold: self.success_threshold,
            tcp_socket: crate::OptionableConvert::into_optioned(self.tcp_socket),
            termination_grace_period_seconds: self.termination_grace_period_seconds,
            timeout_seconds: self.timeout_seconds,
        }
    }
    fn try_from_optioned(value: ProbeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            exec: crate::OptionableConvert::try_from_optioned(value.exec)?,
            failure_threshold: value.failure_threshold,
            grpc: crate::OptionableConvert::try_from_optioned(value.grpc)?,
            http_get: crate::OptionableConvert::try_from_optioned(value.http_get)?,
            initial_delay_seconds: value.initial_delay_seconds,
            period_seconds: value.period_seconds,
            success_threshold: value.success_threshold,
            tcp_socket: crate::OptionableConvert::try_from_optioned(value.tcp_socket)?,
            termination_grace_period_seconds: value.termination_grace_period_seconds,
            timeout_seconds: value.timeout_seconds,
        })
    }
    fn merge(&mut self, other: ProbeAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.exec, other.exec)?;
        self.failure_threshold = other.failure_threshold;
        crate::OptionableConvert::merge(&mut self.grpc, other.grpc)?;
        crate::OptionableConvert::merge(&mut self.http_get, other.http_get)?;
        self.initial_delay_seconds = other.initial_delay_seconds;
        self.period_seconds = other.period_seconds;
        self.success_threshold = other.success_threshold;
        crate::OptionableConvert::merge(&mut self.tcp_socket, other.tcp_socket)?;
        self.termination_grace_period_seconds = other.termination_grace_period_seconds;
        self.timeout_seconds = other.timeout_seconds;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Probe> for ProbeAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Probe) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Probe, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Probe,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
