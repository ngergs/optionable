pub struct LifecycleHandlerOpt {
    pub exec: <Option<
        ::k8s_openapi::api::core::v1::ExecAction,
    > as crate::Optionable>::Optioned,
    pub http_get: <Option<
        ::k8s_openapi::api::core::v1::HTTPGetAction,
    > as crate::Optionable>::Optioned,
    pub sleep: <Option<
        ::k8s_openapi::api::core::v1::SleepAction,
    > as crate::Optionable>::Optioned,
    pub tcp_socket: <Option<
        ::k8s_openapi::api::core::v1::TCPSocketAction,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::lifecycle_handler::LifecycleHandler {
    type Optioned = LifecycleHandlerOpt;
}
#[automatically_derived]
impl crate::Optionable for LifecycleHandlerOpt {
    type Optioned = LifecycleHandlerOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::lifecycle_handler::LifecycleHandler {
    fn into_optioned(self) -> LifecycleHandlerOpt {
        LifecycleHandlerOpt {
            exec: <Option<
                ::k8s_openapi::api::core::v1::ExecAction,
            > as crate::OptionableConvert>::into_optioned(self.exec),
            http_get: <Option<
                ::k8s_openapi::api::core::v1::HTTPGetAction,
            > as crate::OptionableConvert>::into_optioned(self.http_get),
            sleep: <Option<
                ::k8s_openapi::api::core::v1::SleepAction,
            > as crate::OptionableConvert>::into_optioned(self.sleep),
            tcp_socket: <Option<
                ::k8s_openapi::api::core::v1::TCPSocketAction,
            > as crate::OptionableConvert>::into_optioned(self.tcp_socket),
        }
    }
    fn try_from_optioned(
        value: LifecycleHandlerOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            exec: <Option<
                ::k8s_openapi::api::core::v1::ExecAction,
            > as crate::OptionableConvert>::try_from_optioned(value.exec)?,
            http_get: <Option<
                ::k8s_openapi::api::core::v1::HTTPGetAction,
            > as crate::OptionableConvert>::try_from_optioned(value.http_get)?,
            sleep: <Option<
                ::k8s_openapi::api::core::v1::SleepAction,
            > as crate::OptionableConvert>::try_from_optioned(value.sleep)?,
            tcp_socket: <Option<
                ::k8s_openapi::api::core::v1::TCPSocketAction,
            > as crate::OptionableConvert>::try_from_optioned(value.tcp_socket)?,
        })
    }
    fn merge(
        &mut self,
        other: LifecycleHandlerOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ExecAction,
        > as crate::OptionableConvert>::merge(&mut self.exec, other.exec)?;
        <Option<
            ::k8s_openapi::api::core::v1::HTTPGetAction,
        > as crate::OptionableConvert>::merge(&mut self.http_get, other.http_get)?;
        <Option<
            ::k8s_openapi::api::core::v1::SleepAction,
        > as crate::OptionableConvert>::merge(&mut self.sleep, other.sleep)?;
        <Option<
            ::k8s_openapi::api::core::v1::TCPSocketAction,
        > as crate::OptionableConvert>::merge(&mut self.tcp_socket, other.tcp_socket)?;
        Ok(())
    }
}
