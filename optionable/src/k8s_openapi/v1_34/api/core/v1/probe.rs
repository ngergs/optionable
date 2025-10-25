pub struct ProbeOpt {
    pub exec: <Option<
        ::k8s_openapi::api::core::v1::ExecAction,
    > as crate::Optionable>::Optioned,
    pub failure_threshold: <Option<i32> as crate::Optionable>::Optioned,
    pub grpc: <Option<
        ::k8s_openapi::api::core::v1::GRPCAction,
    > as crate::Optionable>::Optioned,
    pub http_get: <Option<
        ::k8s_openapi::api::core::v1::HTTPGetAction,
    > as crate::Optionable>::Optioned,
    pub initial_delay_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub period_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub success_threshold: <Option<i32> as crate::Optionable>::Optioned,
    pub tcp_socket: <Option<
        ::k8s_openapi::api::core::v1::TCPSocketAction,
    > as crate::Optionable>::Optioned,
    pub termination_grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub timeout_seconds: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::probe::Probe {
    type Optioned = ProbeOpt;
}
#[automatically_derived]
impl crate::Optionable for ProbeOpt {
    type Optioned = ProbeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::probe::Probe {
    fn into_optioned(self) -> ProbeOpt {
        ProbeOpt {
            exec: <Option<
                ::k8s_openapi::api::core::v1::ExecAction,
            > as crate::OptionableConvert>::into_optioned(self.exec),
            failure_threshold: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.failure_threshold),
            grpc: <Option<
                ::k8s_openapi::api::core::v1::GRPCAction,
            > as crate::OptionableConvert>::into_optioned(self.grpc),
            http_get: <Option<
                ::k8s_openapi::api::core::v1::HTTPGetAction,
            > as crate::OptionableConvert>::into_optioned(self.http_get),
            initial_delay_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.initial_delay_seconds),
            period_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.period_seconds),
            success_threshold: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.success_threshold),
            tcp_socket: <Option<
                ::k8s_openapi::api::core::v1::TCPSocketAction,
            > as crate::OptionableConvert>::into_optioned(self.tcp_socket),
            termination_grace_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(
                self.termination_grace_period_seconds,
            ),
            timeout_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.timeout_seconds),
        }
    }
    fn try_from_optioned(value: ProbeOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exec: <Option<
                ::k8s_openapi::api::core::v1::ExecAction,
            > as crate::OptionableConvert>::try_from_optioned(value.exec)?,
            failure_threshold: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.failure_threshold)?,
            grpc: <Option<
                ::k8s_openapi::api::core::v1::GRPCAction,
            > as crate::OptionableConvert>::try_from_optioned(value.grpc)?,
            http_get: <Option<
                ::k8s_openapi::api::core::v1::HTTPGetAction,
            > as crate::OptionableConvert>::try_from_optioned(value.http_get)?,
            initial_delay_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.initial_delay_seconds,
            )?,
            period_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.period_seconds)?,
            success_threshold: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.success_threshold)?,
            tcp_socket: <Option<
                ::k8s_openapi::api::core::v1::TCPSocketAction,
            > as crate::OptionableConvert>::try_from_optioned(value.tcp_socket)?,
            termination_grace_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.termination_grace_period_seconds,
            )?,
            timeout_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.timeout_seconds)?,
        })
    }
    fn merge(&mut self, other: ProbeOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ExecAction,
        > as crate::OptionableConvert>::merge(&mut self.exec, other.exec)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.failure_threshold,
            other.failure_threshold,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::GRPCAction,
        > as crate::OptionableConvert>::merge(&mut self.grpc, other.grpc)?;
        <Option<
            ::k8s_openapi::api::core::v1::HTTPGetAction,
        > as crate::OptionableConvert>::merge(&mut self.http_get, other.http_get)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.initial_delay_seconds,
            other.initial_delay_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.period_seconds,
            other.period_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.success_threshold,
            other.success_threshold,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::TCPSocketAction,
        > as crate::OptionableConvert>::merge(&mut self.tcp_socket, other.tcp_socket)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.termination_grace_period_seconds,
            other.termination_grace_period_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        )?;
        Ok(())
    }
}
