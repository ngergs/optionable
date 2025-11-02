#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientIPConfigAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ClientIPConfig {
    type Optioned = ClientIPConfigAc;
}
#[automatically_derived]
impl crate::Optionable for ClientIPConfigAc {
    type Optioned = ClientIPConfigAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ClientIPConfig {
    fn into_optioned(self) -> ClientIPConfigAc {
        ClientIPConfigAc {
            timeout_seconds: crate::OptionableConvert::into_optioned(
                self.timeout_seconds,
            ),
        }
    }
    fn try_from_optioned(
        value: ClientIPConfigAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            timeout_seconds: crate::OptionableConvert::try_from_optioned(
                value.timeout_seconds,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ClientIPConfigAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.timeout_seconds,
            other.timeout_seconds,
        )?;
        Ok(())
    }
}
