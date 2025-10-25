pub struct LifecycleOpt {
    pub post_start: <Option<
        ::k8s_openapi::api::core::v1::LifecycleHandler,
    > as crate::Optionable>::Optioned,
    pub pre_stop: <Option<
        ::k8s_openapi::api::core::v1::LifecycleHandler,
    > as crate::Optionable>::Optioned,
    pub stop_signal: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Lifecycle {
    type Optioned = LifecycleOpt;
}
#[automatically_derived]
impl crate::Optionable for LifecycleOpt {
    type Optioned = LifecycleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Lifecycle {
    fn into_optioned(self) -> LifecycleOpt {
        LifecycleOpt {
            post_start: <Option<
                ::k8s_openapi::api::core::v1::LifecycleHandler,
            > as crate::OptionableConvert>::into_optioned(self.post_start),
            pre_stop: <Option<
                ::k8s_openapi::api::core::v1::LifecycleHandler,
            > as crate::OptionableConvert>::into_optioned(self.pre_stop),
            stop_signal: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.stop_signal),
        }
    }
    fn try_from_optioned(value: LifecycleOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            post_start: <Option<
                ::k8s_openapi::api::core::v1::LifecycleHandler,
            > as crate::OptionableConvert>::try_from_optioned(value.post_start)?,
            pre_stop: <Option<
                ::k8s_openapi::api::core::v1::LifecycleHandler,
            > as crate::OptionableConvert>::try_from_optioned(value.pre_stop)?,
            stop_signal: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.stop_signal)?,
        })
    }
    fn merge(&mut self, other: LifecycleOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::LifecycleHandler,
        > as crate::OptionableConvert>::merge(&mut self.post_start, other.post_start)?;
        <Option<
            ::k8s_openapi::api::core::v1::LifecycleHandler,
        > as crate::OptionableConvert>::merge(&mut self.pre_stop, other.pre_stop)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.stop_signal, other.stop_signal)?;
        Ok(())
    }
}
