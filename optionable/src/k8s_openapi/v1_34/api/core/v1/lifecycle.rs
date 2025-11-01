pub struct LifecycleAc {
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
    type Optioned = LifecycleAc;
}
#[automatically_derived]
impl crate::Optionable for LifecycleAc {
    type Optioned = LifecycleAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Lifecycle {
    fn into_optioned(self) -> LifecycleAc {
        LifecycleAc {
            post_start: crate::OptionableConvert::into_optioned(self.post_start),
            pre_stop: crate::OptionableConvert::into_optioned(self.pre_stop),
            stop_signal: crate::OptionableConvert::into_optioned(self.stop_signal),
        }
    }
    fn try_from_optioned(value: LifecycleAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            post_start: crate::OptionableConvert::try_from_optioned(value.post_start)?,
            pre_stop: crate::OptionableConvert::try_from_optioned(value.pre_stop)?,
            stop_signal: crate::OptionableConvert::try_from_optioned(value.stop_signal)?,
        })
    }
    fn merge(&mut self, other: LifecycleAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.post_start, other.post_start)?;
        crate::OptionableConvert::merge(&mut self.pre_stop, other.pre_stop)?;
        crate::OptionableConvert::merge(&mut self.stop_signal, other.stop_signal)?;
        Ok(())
    }
}
