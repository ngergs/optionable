pub struct SessionAffinityConfigOpt {
    pub client_ip: <Option<
        ::k8s_openapi::api::core::v1::ClientIPConfig,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::session_affinity_config::SessionAffinityConfig {
    type Optioned = SessionAffinityConfigOpt;
}
#[automatically_derived]
impl crate::Optionable for SessionAffinityConfigOpt {
    type Optioned = SessionAffinityConfigOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::session_affinity_config::SessionAffinityConfig {
    fn into_optioned(self) -> SessionAffinityConfigOpt {
        SessionAffinityConfigOpt {
            client_ip: <Option<
                ::k8s_openapi::api::core::v1::ClientIPConfig,
            > as crate::OptionableConvert>::into_optioned(self.client_ip),
        }
    }
    fn try_from_optioned(
        value: SessionAffinityConfigOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            client_ip: <Option<
                ::k8s_openapi::api::core::v1::ClientIPConfig,
            > as crate::OptionableConvert>::try_from_optioned(value.client_ip)?,
        })
    }
    fn merge(
        &mut self,
        other: SessionAffinityConfigOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ClientIPConfig,
        > as crate::OptionableConvert>::merge(&mut self.client_ip, other.client_ip)?;
        Ok(())
    }
}
