#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents a volume that is populated with the contents of a git repository. Git repo volumes do not support ownership management. Git repo volumes support SELinux relabeling.
///
/// DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GitRepoVolumeSourceAc {
    /// directory is the target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<std::string::String>,
    /// repository is the URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<std::string::String>,
    /// revision is the commit hash for the specified revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::GitRepoVolumeSource {
    type Optioned = GitRepoVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GitRepoVolumeSourceAc {
    type Optioned = GitRepoVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::GitRepoVolumeSource {
    fn into_optioned(self) -> GitRepoVolumeSourceAc {
        GitRepoVolumeSourceAc {
            directory: self.directory,
            repository: Some(self.repository),
            revision: self.revision,
        }
    }
    fn try_from_optioned(value: GitRepoVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            directory: value.directory,
            repository: value
                .repository
                .ok_or(crate::Error {
                    missing_field: "repository",
                })?,
            revision: value.revision,
        })
    }
    fn merge(&mut self, other: GitRepoVolumeSourceAc) -> Result<(), crate::Error> {
        self.directory = other.directory;
        if let Some(other_value) = other.repository {
            self.repository = other_value;
        }
        self.revision = other.revision;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::GitRepoVolumeSource>
for GitRepoVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::GitRepoVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::GitRepoVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::GitRepoVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
