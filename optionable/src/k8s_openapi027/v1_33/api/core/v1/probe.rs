#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Probe describes a health check to be performed against a container to determine whether it is alive or ready to receive traffic.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProbeAc {
    /// Exec specifies a command to execute in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<
        <::k8s_openapi027::api::core::v1::ExecAction as crate::Optionable>::Optioned,
    >,
    /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    /// GRPC specifies a GRPC HealthCheckRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc: Option<
        <::k8s_openapi027::api::core::v1::GRPCAction as crate::Optionable>::Optioned,
    >,
    /// HTTPGet specifies an HTTP GET request to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<
        <::k8s_openapi027::api::core::v1::HTTPGetAction as crate::Optionable>::Optioned,
    >,
    /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    /// TCPSocket specifies a connection to a TCP port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<
        <::k8s_openapi027::api::core::v1::TCPSocketAction as crate::Optionable>::Optioned,
    >,
    /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_grace_period_seconds: Option<i64>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
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
        if self.exec.is_none() {
            self.exec = crate::OptionableConvert::try_from_optioned(other.exec)?;
        } else if let Some(self_value) = self.exec.as_mut()
            && let Some(other_value) = other.exec
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.failure_threshold.is_none() {
            self.failure_threshold = crate::OptionableConvert::try_from_optioned(
                other.failure_threshold,
            )?;
        } else if let Some(self_value) = self.failure_threshold.as_mut()
            && let Some(other_value) = other.failure_threshold
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.grpc.is_none() {
            self.grpc = crate::OptionableConvert::try_from_optioned(other.grpc)?;
        } else if let Some(self_value) = self.grpc.as_mut()
            && let Some(other_value) = other.grpc
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.http_get.is_none() {
            self.http_get = crate::OptionableConvert::try_from_optioned(other.http_get)?;
        } else if let Some(self_value) = self.http_get.as_mut()
            && let Some(other_value) = other.http_get
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.initial_delay_seconds.is_none() {
            self.initial_delay_seconds = crate::OptionableConvert::try_from_optioned(
                other.initial_delay_seconds,
            )?;
        } else if let Some(self_value) = self.initial_delay_seconds.as_mut()
            && let Some(other_value) = other.initial_delay_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.period_seconds.is_none() {
            self.period_seconds = crate::OptionableConvert::try_from_optioned(
                other.period_seconds,
            )?;
        } else if let Some(self_value) = self.period_seconds.as_mut()
            && let Some(other_value) = other.period_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.success_threshold.is_none() {
            self.success_threshold = crate::OptionableConvert::try_from_optioned(
                other.success_threshold,
            )?;
        } else if let Some(self_value) = self.success_threshold.as_mut()
            && let Some(other_value) = other.success_threshold
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.tcp_socket.is_none() {
            self.tcp_socket = crate::OptionableConvert::try_from_optioned(
                other.tcp_socket,
            )?;
        } else if let Some(self_value) = self.tcp_socket.as_mut()
            && let Some(other_value) = other.tcp_socket
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.termination_grace_period_seconds.is_none() {
            self.termination_grace_period_seconds = crate::OptionableConvert::try_from_optioned(
                other.termination_grace_period_seconds,
            )?;
        } else if let Some(self_value) = self.termination_grace_period_seconds.as_mut()
            && let Some(other_value) = other.termination_grace_period_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.timeout_seconds.is_none() {
            self.timeout_seconds = crate::OptionableConvert::try_from_optioned(
                other.timeout_seconds,
            )?;
        } else if let Some(self_value) = self.timeout_seconds.as_mut()
            && let Some(other_value) = other.timeout_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for ProbeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.exec, other.exec);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.failure_threshold,
            other.failure_threshold,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.grpc, other.grpc);
        k8s_openapi027::DeepMerge::merge_from(&mut self.http_get, other.http_get);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.initial_delay_seconds,
            other.initial_delay_seconds,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.period_seconds,
            other.period_seconds,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.success_threshold,
            other.success_threshold,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.tcp_socket, other.tcp_socket);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.termination_grace_period_seconds,
            other.termination_grace_period_seconds,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        );
    }
}
