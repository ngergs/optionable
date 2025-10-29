pub struct NFSVolumeSourceOpt {
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub server: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NFSVolumeSource {
    type Optioned = NFSVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for NFSVolumeSourceOpt {
    type Optioned = NFSVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NFSVolumeSource {
    fn into_optioned(self) -> NFSVolumeSourceOpt {
        NFSVolumeSourceOpt {
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            server: Some(crate::OptionableConvert::into_optioned(self.server)),
        }
    }
    fn try_from_optioned(
        value: NFSVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            server: crate::OptionableConvert::try_from_optioned(
                value
                    .server
                    .ok_or(crate::optionable::Error {
                        missing_field: "server",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NFSVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.server {
            crate::OptionableConvert::merge(&mut self.server, other_value)?;
        }
        Ok(())
    }
}
