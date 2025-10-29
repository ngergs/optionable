pub struct GitRepoVolumeSourceOpt {
    pub directory: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub repository: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub revision: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GitRepoVolumeSource {
    type Optioned = GitRepoVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for GitRepoVolumeSourceOpt {
    type Optioned = GitRepoVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::GitRepoVolumeSource {
    fn into_optioned(self) -> GitRepoVolumeSourceOpt {
        GitRepoVolumeSourceOpt {
            directory: crate::OptionableConvert::into_optioned(self.directory),
            repository: Some(crate::OptionableConvert::into_optioned(self.repository)),
            revision: crate::OptionableConvert::into_optioned(self.revision),
        }
    }
    fn try_from_optioned(
        value: GitRepoVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            directory: crate::OptionableConvert::try_from_optioned(value.directory)?,
            repository: crate::OptionableConvert::try_from_optioned(
                value
                    .repository
                    .ok_or(crate::optionable::Error {
                        missing_field: "repository",
                    })?,
            )?,
            revision: crate::OptionableConvert::try_from_optioned(value.revision)?,
        })
    }
    fn merge(
        &mut self,
        other: GitRepoVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.directory, other.directory)?;
        if let Some(other_value) = other.repository {
            crate::OptionableConvert::merge(&mut self.repository, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.revision, other.revision)?;
        Ok(())
    }
}
