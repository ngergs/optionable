pub struct AzureFilePersistentVolumeSourceOpt {
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub secret_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub secret_namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub share_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource {
    type Optioned = AzureFilePersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for AzureFilePersistentVolumeSourceOpt {
    type Optioned = AzureFilePersistentVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::AzureFilePersistentVolumeSource {
    fn into_optioned(self) -> AzureFilePersistentVolumeSourceOpt {
        AzureFilePersistentVolumeSourceOpt {
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            secret_name: Some(crate::OptionableConvert::into_optioned(self.secret_name)),
            secret_namespace: crate::OptionableConvert::into_optioned(
                self.secret_namespace,
            ),
            share_name: Some(crate::OptionableConvert::into_optioned(self.share_name)),
        }
    }
    fn try_from_optioned(
        value: AzureFilePersistentVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            secret_name: crate::OptionableConvert::try_from_optioned(
                value
                    .secret_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "secret_name",
                    })?,
            )?,
            secret_namespace: crate::OptionableConvert::try_from_optioned(
                value.secret_namespace,
            )?,
            share_name: crate::OptionableConvert::try_from_optioned(
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
        other: AzureFilePersistentVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        if let Some(other_value) = other.secret_name {
            crate::OptionableConvert::merge(&mut self.secret_name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.secret_namespace,
            other.secret_namespace,
        )?;
        if let Some(other_value) = other.share_name {
            crate::OptionableConvert::merge(&mut self.share_name, other_value)?;
        }
        Ok(())
    }
}
