pub struct ExecActionOpt {
    pub command: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ExecAction {
    type Optioned = ExecActionOpt;
}
#[automatically_derived]
impl crate::Optionable for ExecActionOpt {
    type Optioned = ExecActionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ExecAction {
    fn into_optioned(self) -> ExecActionOpt {
        ExecActionOpt {
            command: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.command),
        }
    }
    fn try_from_optioned(
        value: ExecActionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            command: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.command)?,
        })
    }
    fn merge(&mut self, other: ExecActionOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.command, other.command)?;
        Ok(())
    }
}
