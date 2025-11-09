#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ProbeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: <Option<
        ::k8s_openapi::api::core::v1::ExecAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: <Option<
        ::k8s_openapi::api::core::v1::GRPCAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: <Option<
        ::k8s_openapi::api::core::v1::HTTPGetAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_threshold: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: <Option<
        ::k8s_openapi::api::core::v1::TCPSocketAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Probe {
    type Optioned = ProbeAc;
}
#[automatically_derived]
impl crate::Optionable for ProbeAc {
    type Optioned = ProbeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Probe {
    fn into_optioned(self) -> ProbeAc {
        ProbeAc {
            exec: crate::OptionableConvert::into_optioned(self.exec),
            failure_threshold: crate::OptionableConvert::into_optioned(
                self.failure_threshold,
            ),
            grpc: crate::OptionableConvert::into_optioned(self.grpc),
            http_get: crate::OptionableConvert::into_optioned(self.http_get),
            initial_delay_seconds: crate::OptionableConvert::into_optioned(
                self.initial_delay_seconds,
            ),
            period_seconds: crate::OptionableConvert::into_optioned(self.period_seconds),
            success_threshold: crate::OptionableConvert::into_optioned(
                self.success_threshold,
            ),
            tcp_socket: crate::OptionableConvert::into_optioned(self.tcp_socket),
            termination_grace_period_seconds: crate::OptionableConvert::into_optioned(
                self.termination_grace_period_seconds,
            ),
            timeout_seconds: crate::OptionableConvert::into_optioned(
                self.timeout_seconds,
            ),
        }
    }
    fn try_from_optioned(value: ProbeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            exec: crate::OptionableConvert::try_from_optioned(value.exec)?,
            failure_threshold: crate::OptionableConvert::try_from_optioned(
                value.failure_threshold,
            )?,
            grpc: crate::OptionableConvert::try_from_optioned(value.grpc)?,
            http_get: crate::OptionableConvert::try_from_optioned(value.http_get)?,
            initial_delay_seconds: crate::OptionableConvert::try_from_optioned(
                value.initial_delay_seconds,
            )?,
            period_seconds: crate::OptionableConvert::try_from_optioned(
                value.period_seconds,
            )?,
            success_threshold: crate::OptionableConvert::try_from_optioned(
                value.success_threshold,
            )?,
            tcp_socket: crate::OptionableConvert::try_from_optioned(value.tcp_socket)?,
            termination_grace_period_seconds: crate::OptionableConvert::try_from_optioned(
                value.termination_grace_period_seconds,
            )?,
            timeout_seconds: crate::OptionableConvert::try_from_optioned(
                value.timeout_seconds,
            )?,
        })
    }
    fn merge(&mut self, other: ProbeAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.exec, other.exec)?;
        crate::OptionableConvert::merge(
            &mut self.failure_threshold,
            other.failure_threshold,
        )?;
        crate::OptionableConvert::merge(&mut self.grpc, other.grpc)?;
        crate::OptionableConvert::merge(&mut self.http_get, other.http_get)?;
        crate::OptionableConvert::merge(
            &mut self.initial_delay_seconds,
            other.initial_delay_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.period_seconds, other.period_seconds)?;
        crate::OptionableConvert::merge(
            &mut self.success_threshold,
            other.success_threshold,
        )?;
        crate::OptionableConvert::merge(&mut self.tcp_socket, other.tcp_socket)?;
        crate::OptionableConvert::merge(
            &mut self.termination_grace_period_seconds,
            other.termination_grace_period_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        )?;
        Ok(())
    }
}
