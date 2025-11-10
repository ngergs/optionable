#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct GitRepoVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GitRepoVolumeSource {
    type Optioned = GitRepoVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GitRepoVolumeSourceAc {
    type Optioned = GitRepoVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::GitRepoVolumeSource {
    fn into_optioned(self) -> GitRepoVolumeSourceAc {
        GitRepoVolumeSourceAc {
            directory: crate::OptionableConvert::into_optioned(self.directory),
            repository: Some(crate::OptionableConvert::into_optioned(self.repository)),
            revision: crate::OptionableConvert::into_optioned(self.revision),
        }
    }
    fn try_from_optioned(value: GitRepoVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            directory: crate::OptionableConvert::try_from_optioned(value.directory)?,
            repository: crate::OptionableConvert::try_from_optioned(
                value
                    .repository
                    .ok_or(crate::Error {
                        missing_field: "repository",
                    })?,
            )?,
            revision: crate::OptionableConvert::try_from_optioned(value.revision)?,
        })
    }
    fn merge(&mut self, other: GitRepoVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.directory, other.directory)?;
        if let Some(other_value) = other.repository {
            crate::OptionableConvert::merge(&mut self.repository, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.revision, other.revision)?;
        Ok(())
    }
}
