#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LifecycleHandler defines a specific action that should be taken in a lifecycle hook. One and only one of the fields, except TCPSocket must be specified.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LifecycleHandlerAc {
    /// Exec specifies the action to take.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: Option<
        <::k8s_openapi027::api::core::v1::ExecAction as crate::Optionable>::Optioned,
    >,
    /// HTTPGet specifies the http request to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: Option<
        <::k8s_openapi027::api::core::v1::HTTPGetAction as crate::Optionable>::Optioned,
    >,
    /// Sleep represents the duration that the container should sleep before being terminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep: Option<
        <::k8s_openapi027::api::core::v1::SleepAction as crate::Optionable>::Optioned,
    >,
    /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: Option<
        <::k8s_openapi027::api::core::v1::TCPSocketAction as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::LifecycleHandler {
    type Optioned = LifecycleHandlerAc;
}
#[automatically_derived]
impl crate::Optionable for LifecycleHandlerAc {
    type Optioned = LifecycleHandlerAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::LifecycleHandler {
    fn into_optioned(self) -> LifecycleHandlerAc {
        LifecycleHandlerAc {
            exec: crate::OptionableConvert::into_optioned(self.exec),
            http_get: crate::OptionableConvert::into_optioned(self.http_get),
            sleep: crate::OptionableConvert::into_optioned(self.sleep),
            tcp_socket: crate::OptionableConvert::into_optioned(self.tcp_socket),
        }
    }
    fn try_from_optioned(value: LifecycleHandlerAc) -> Result<Self, crate::Error> {
        Ok(Self {
            exec: crate::OptionableConvert::try_from_optioned(value.exec)?,
            http_get: crate::OptionableConvert::try_from_optioned(value.http_get)?,
            sleep: crate::OptionableConvert::try_from_optioned(value.sleep)?,
            tcp_socket: crate::OptionableConvert::try_from_optioned(value.tcp_socket)?,
        })
    }
    fn merge(&mut self, other: LifecycleHandlerAc) -> Result<(), crate::Error> {
        if self.exec.is_none() {
            self.exec = crate::OptionableConvert::try_from_optioned(other.exec)?;
        } else if let Some(self_value) = self.exec.as_mut()
            && let Some(other_value) = other.exec
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
        if self.sleep.is_none() {
            self.sleep = crate::OptionableConvert::try_from_optioned(other.sleep)?;
        } else if let Some(self_value) = self.sleep.as_mut()
            && let Some(other_value) = other.sleep
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
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::LifecycleHandler>
for LifecycleHandlerAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::LifecycleHandler) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::LifecycleHandler, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::LifecycleHandler,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for LifecycleHandlerAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.exec, other.exec);
        k8s_openapi027::DeepMerge::merge_from(&mut self.http_get, other.http_get);
        k8s_openapi027::DeepMerge::merge_from(&mut self.sleep, other.sleep);
        k8s_openapi027::DeepMerge::merge_from(&mut self.tcp_socket, other.tcp_socket);
    }
}
