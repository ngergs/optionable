pub struct ClientIPConfigOpt {
    pub timeout_seconds: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ClientIPConfig {
    type Optioned = ClientIPConfigOpt;
}
#[automatically_derived]
impl crate::Optionable for ClientIPConfigOpt {
    type Optioned = ClientIPConfigOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ClientIPConfig {
    fn into_optioned(self) -> ClientIPConfigOpt {
        ClientIPConfigOpt {
            timeout_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.timeout_seconds),
        }
    }
    fn try_from_optioned(
        value: ClientIPConfigOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            timeout_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.timeout_seconds)?,
        })
    }
    fn merge(
        &mut self,
        other: ClientIPConfigOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        )?;
        Ok(())
    }
}
