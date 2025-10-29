pub struct QuobyteVolumeSourceOpt {
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub registry: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub tenant: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub volume: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::QuobyteVolumeSource {
    type Optioned = QuobyteVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for QuobyteVolumeSourceOpt {
    type Optioned = QuobyteVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::QuobyteVolumeSource {
    fn into_optioned(self) -> QuobyteVolumeSourceOpt {
        QuobyteVolumeSourceOpt {
            group: crate::OptionableConvert::into_optioned(self.group),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            registry: Some(crate::OptionableConvert::into_optioned(self.registry)),
            tenant: crate::OptionableConvert::into_optioned(self.tenant),
            user: crate::OptionableConvert::into_optioned(self.user),
            volume: Some(crate::OptionableConvert::into_optioned(self.volume)),
        }
    }
    fn try_from_optioned(
        value: QuobyteVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            registry: crate::OptionableConvert::try_from_optioned(
                value
                    .registry
                    .ok_or(crate::optionable::Error {
                        missing_field: "registry",
                    })?,
            )?,
            tenant: crate::OptionableConvert::try_from_optioned(value.tenant)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
            volume: crate::OptionableConvert::try_from_optioned(
                value
                    .volume
                    .ok_or(crate::optionable::Error {
                        missing_field: "volume",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: QuobyteVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.registry {
            crate::OptionableConvert::merge(&mut self.registry, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.tenant, other.tenant)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        if let Some(other_value) = other.volume {
            crate::OptionableConvert::merge(&mut self.volume, other_value)?;
        }
        Ok(())
    }
}
