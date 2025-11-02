#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifecycleHandlerAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec: <Option<
        ::k8s_openapi::api::core::v1::ExecAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_get: <Option<
        ::k8s_openapi::api::core::v1::HTTPGetAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep: <Option<
        ::k8s_openapi::api::core::v1::SleepAction,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_socket: <Option<
        ::k8s_openapi::api::core::v1::TCPSocketAction,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LifecycleHandler {
    type Optioned = LifecycleHandlerAc;
}
#[automatically_derived]
impl crate::Optionable for LifecycleHandlerAc {
    type Optioned = LifecycleHandlerAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LifecycleHandler {
    fn into_optioned(self) -> LifecycleHandlerAc {
        LifecycleHandlerAc {
            exec: crate::OptionableConvert::into_optioned(self.exec),
            http_get: crate::OptionableConvert::into_optioned(self.http_get),
            sleep: crate::OptionableConvert::into_optioned(self.sleep),
            tcp_socket: crate::OptionableConvert::into_optioned(self.tcp_socket),
        }
    }
    fn try_from_optioned(
        value: LifecycleHandlerAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exec: crate::OptionableConvert::try_from_optioned(value.exec)?,
            http_get: crate::OptionableConvert::try_from_optioned(value.http_get)?,
            sleep: crate::OptionableConvert::try_from_optioned(value.sleep)?,
            tcp_socket: crate::OptionableConvert::try_from_optioned(value.tcp_socket)?,
        })
    }
    fn merge(
        &mut self,
        other: LifecycleHandlerAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.exec, other.exec)?;
        crate::OptionableConvert::merge(&mut self.http_get, other.http_get)?;
        crate::OptionableConvert::merge(&mut self.sleep, other.sleep)?;
        crate::OptionableConvert::merge(&mut self.tcp_socket, other.tcp_socket)?;
        Ok(())
    }
}
