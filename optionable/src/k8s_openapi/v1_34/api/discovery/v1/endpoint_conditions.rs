pub struct EndpointConditionsOpt {
    pub ready: <Option<bool> as crate::Optionable>::Optioned,
    pub serving: <Option<bool> as crate::Optionable>::Optioned,
    pub terminating: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::EndpointConditions {
    type Optioned = EndpointConditionsOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointConditionsOpt {
    type Optioned = EndpointConditionsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::EndpointConditions {
    fn into_optioned(self) -> EndpointConditionsOpt {
        EndpointConditionsOpt {
            ready: <Option<bool> as crate::OptionableConvert>::into_optioned(self.ready),
            serving: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.serving),
            terminating: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.terminating),
        }
    }
    fn try_from_optioned(
        value: EndpointConditionsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ready: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.ready)?,
            serving: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.serving)?,
            terminating: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.terminating)?,
        })
    }
    fn merge(
        &mut self,
        other: EndpointConditionsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<bool> as crate::OptionableConvert>::merge(&mut self.ready, other.ready)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.serving, other.serving)?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.terminating, other.terminating)?;
        Ok(())
    }
}
