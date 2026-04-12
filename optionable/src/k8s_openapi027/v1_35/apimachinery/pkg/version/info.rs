#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Info contains versioning information. how we'll want to distribute that information.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InfoAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_date: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<std::string::String>,
    /// EmulationMajor is the major version of the emulation version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulation_major: Option<std::string::String>,
    /// EmulationMinor is the minor version of the emulation version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulation_minor: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_tree_state: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_version: Option<std::string::String>,
    /// Major is the major version of the binary version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<std::string::String>,
    /// MinCompatibilityMajor is the major version of the minimum compatibility version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_compatibility_major: Option<std::string::String>,
    /// MinCompatibilityMinor is the minor version of the minimum compatibility version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_compatibility_minor: Option<std::string::String>,
    /// Minor is the minor version of the binary version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::version::Info {
    type Optioned = InfoAc;
}
#[automatically_derived]
impl crate::Optionable for InfoAc {
    type Optioned = InfoAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::apimachinery::pkg::version::Info {
    fn into_optioned(self) -> InfoAc {
        InfoAc {
            build_date: Some(self.build_date),
            compiler: Some(self.compiler),
            emulation_major: self.emulation_major,
            emulation_minor: self.emulation_minor,
            git_commit: Some(self.git_commit),
            git_tree_state: Some(self.git_tree_state),
            git_version: Some(self.git_version),
            go_version: Some(self.go_version),
            major: Some(self.major),
            min_compatibility_major: self.min_compatibility_major,
            min_compatibility_minor: self.min_compatibility_minor,
            minor: Some(self.minor),
            platform: Some(self.platform),
        }
    }
    fn try_from_optioned(value: InfoAc) -> Result<Self, crate::Error> {
        Ok(Self {
            build_date: value
                .build_date
                .ok_or(crate::Error {
                    missing_field: "build_date",
                })?,
            compiler: value
                .compiler
                .ok_or(crate::Error {
                    missing_field: "compiler",
                })?,
            emulation_major: value.emulation_major,
            emulation_minor: value.emulation_minor,
            git_commit: value
                .git_commit
                .ok_or(crate::Error {
                    missing_field: "git_commit",
                })?,
            git_tree_state: value
                .git_tree_state
                .ok_or(crate::Error {
                    missing_field: "git_tree_state",
                })?,
            git_version: value
                .git_version
                .ok_or(crate::Error {
                    missing_field: "git_version",
                })?,
            go_version: value
                .go_version
                .ok_or(crate::Error {
                    missing_field: "go_version",
                })?,
            major: value
                .major
                .ok_or(crate::Error {
                    missing_field: "major",
                })?,
            min_compatibility_major: value.min_compatibility_major,
            min_compatibility_minor: value.min_compatibility_minor,
            minor: value
                .minor
                .ok_or(crate::Error {
                    missing_field: "minor",
                })?,
            platform: value
                .platform
                .ok_or(crate::Error {
                    missing_field: "platform",
                })?,
        })
    }
    fn merge(&mut self, other: InfoAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.build_date {
            self.build_date = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.compiler {
            self.compiler = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.emulation_major.is_none() {
            self.emulation_major = crate::OptionableConvert::try_from_optioned(
                other.emulation_major,
            )?;
        } else if let Some(self_value) = self.emulation_major.as_mut()
            && let Some(other_value) = other.emulation_major
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.emulation_minor.is_none() {
            self.emulation_minor = crate::OptionableConvert::try_from_optioned(
                other.emulation_minor,
            )?;
        } else if let Some(self_value) = self.emulation_minor.as_mut()
            && let Some(other_value) = other.emulation_minor
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.git_commit {
            self.git_commit = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.git_tree_state {
            self.git_tree_state = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.git_version {
            self.git_version = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.go_version {
            self.go_version = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.major {
            self.major = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.min_compatibility_major.is_none() {
            self.min_compatibility_major = crate::OptionableConvert::try_from_optioned(
                other.min_compatibility_major,
            )?;
        } else if let Some(self_value) = self.min_compatibility_major.as_mut()
            && let Some(other_value) = other.min_compatibility_major
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.min_compatibility_minor.is_none() {
            self.min_compatibility_minor = crate::OptionableConvert::try_from_optioned(
                other.min_compatibility_minor,
            )?;
        } else if let Some(self_value) = self.min_compatibility_minor.as_mut()
            && let Some(other_value) = other.min_compatibility_minor
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.minor {
            self.minor = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.platform {
            self.platform = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::version::Info>
for InfoAc {
    fn from_optionable(value: k8s_openapi027::apimachinery::pkg::version::Info) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::apimachinery::pkg::version::Info, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::version::Info,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
