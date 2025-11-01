pub struct ContainerStateWaitingAc {
    pub message: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ContainerStateWaiting {
    type Optioned = ContainerStateWaitingAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerStateWaitingAc {
    type Optioned = ContainerStateWaitingAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ContainerStateWaiting {
    fn into_optioned(self) -> ContainerStateWaitingAc {
        ContainerStateWaitingAc {
            message: crate::OptionableConvert::into_optioned(self.message),
            reason: crate::OptionableConvert::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: ContainerStateWaitingAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            message: crate::OptionableConvert::try_from_optioned(value.message)?,
            reason: crate::OptionableConvert::try_from_optioned(value.reason)?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerStateWaitingAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.message, other.message)?;
        crate::OptionableConvert::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
