pub struct ExecActionAc {
    pub command: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ExecAction {
    type Optioned = ExecActionAc;
}
#[automatically_derived]
impl crate::Optionable for ExecActionAc {
    type Optioned = ExecActionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ExecAction {
    fn into_optioned(self) -> ExecActionAc {
        ExecActionAc {
            command: crate::OptionableConvert::into_optioned(self.command),
        }
    }
    fn try_from_optioned(value: ExecActionAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            command: crate::OptionableConvert::try_from_optioned(value.command)?,
        })
    }
    fn merge(&mut self, other: ExecActionAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.command, other.command)?;
        Ok(())
    }
}
