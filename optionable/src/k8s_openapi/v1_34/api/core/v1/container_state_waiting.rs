pub struct ContainerStateWaitingOpt {
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerStateWaiting {
    type Optioned = ContainerStateWaitingOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateWaitingOpt {
    type Optioned = ContainerStateWaitingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerStateWaiting {
    fn into_optioned(self) -> ContainerStateWaitingOpt {
        ContainerStateWaitingOpt {
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.message),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: ContainerStateWaitingOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            message: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.message)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateWaitingOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.message, other.message)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
