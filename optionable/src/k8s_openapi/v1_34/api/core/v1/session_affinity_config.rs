pub struct SessionAffinityConfigAc {
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
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SessionAffinityConfig {
    fn into_optioned(self) -> SessionAffinityConfigAc {
        SessionAffinityConfigAc {
            client_ip: crate::OptionableConvert::into_optioned(self.client_ip),
        }
    }
    fn try_from_optioned(
        value: SessionAffinityConfigAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            client_ip: crate::OptionableConvert::try_from_optioned(value.client_ip)?,
        })
    }
    fn merge(
        &mut self,
        other: SessionAffinityConfigAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.client_ip, other.client_ip)?;
        Ok(())
    }
}
