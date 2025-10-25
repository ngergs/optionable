pub struct AzureFileVolumeSourceOpt {
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub share_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::AzureFileVolumeSource {
    type Optioned = AzureFileVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for AzureFileVolumeSourceOpt {
    type Optioned = AzureFileVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::AzureFileVolumeSource {
    fn into_optioned(self) -> AzureFileVolumeSourceOpt {
        AzureFileVolumeSourceOpt {
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            secret_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.secret_name,
                ),
            ),
            share_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.share_name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: AzureFileVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            secret_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .secret_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "secret_name",
                    })?,
            )?,
            share_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .share_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "share_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: AzureFileVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.secret_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.secret_name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.share_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.share_name,
                other_value,
            )?;
        }
        Ok(())
    }
}
