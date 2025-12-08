#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct SessionAffinityConfigAc {
    #[serde(rename = "clientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: <Option<
        ::k8s_openapi::api::core::v1::ClientIPConfig,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SessionAffinityConfig {
    type Optioned = SessionAffinityConfigAc;
}
#[automatically_derived]
impl crate::Optionable for SessionAffinityConfigAc {
    type Optioned = SessionAffinityConfigAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SessionAffinityConfig {
    fn into_optioned(self) -> SessionAffinityConfigAc {
        SessionAffinityConfigAc {
            client_ip: crate::OptionableConvert::into_optioned(self.client_ip),
        }
    }
    fn try_from_optioned(value: SessionAffinityConfigAc) -> Result<Self, crate::Error> {
        Ok(Self {
            client_ip: crate::OptionableConvert::try_from_optioned(value.client_ip)?,
        })
    }
    fn merge(&mut self, other: SessionAffinityConfigAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.client_ip, other.client_ip)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::SessionAffinityConfig>
for SessionAffinityConfigAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::SessionAffinityConfig,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::SessionAffinityConfig, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::SessionAffinityConfig,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
