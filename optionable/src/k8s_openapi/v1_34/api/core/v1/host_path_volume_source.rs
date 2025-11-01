pub struct HostPathVolumeSourceAc {
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HostPathVolumeSource {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for HostPathVolumeSourceAc {
    type Optioned = HostPathVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HostPathVolumeSource {
    fn into_optioned(self) -> HostPathVolumeSourceAc {
        HostPathVolumeSourceAc {
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            type_: crate::OptionableConvert::into_optioned(self.type_),
        }
    }
    fn try_from_optioned(
        value: HostPathVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
        })
    }
    fn merge(
        &mut self,
        other: HostPathVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        Ok(())
    }
}
