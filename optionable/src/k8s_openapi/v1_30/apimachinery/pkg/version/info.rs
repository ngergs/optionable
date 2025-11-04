#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct InfoAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_date: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_tree_state: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::version::Info {
    type Optioned = InfoAc;
}
#[automatically_derived]
impl crate::Optionable for InfoAc {
    type Optioned = InfoAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::apimachinery::pkg::version::Info {
    fn into_optioned(self) -> InfoAc {
        InfoAc {
            build_date: Some(crate::OptionableConvert::into_optioned(self.build_date)),
            compiler: Some(crate::OptionableConvert::into_optioned(self.compiler)),
            git_commit: Some(crate::OptionableConvert::into_optioned(self.git_commit)),
            git_tree_state: Some(
                crate::OptionableConvert::into_optioned(self.git_tree_state),
            ),
            git_version: Some(crate::OptionableConvert::into_optioned(self.git_version)),
            go_version: Some(crate::OptionableConvert::into_optioned(self.go_version)),
            major: Some(crate::OptionableConvert::into_optioned(self.major)),
            minor: Some(crate::OptionableConvert::into_optioned(self.minor)),
            platform: Some(crate::OptionableConvert::into_optioned(self.platform)),
        }
    }
    fn try_from_optioned(value: InfoAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            build_date: crate::OptionableConvert::try_from_optioned(
                value
                    .build_date
                    .ok_or(crate::optionable::Error {
                        missing_field: "build_date",
                    })?,
            )?,
            compiler: crate::OptionableConvert::try_from_optioned(
                value
                    .compiler
                    .ok_or(crate::optionable::Error {
                        missing_field: "compiler",
                    })?,
            )?,
            git_commit: crate::OptionableConvert::try_from_optioned(
                value
                    .git_commit
                    .ok_or(crate::optionable::Error {
                        missing_field: "git_commit",
                    })?,
            )?,
            git_tree_state: crate::OptionableConvert::try_from_optioned(
                value
                    .git_tree_state
                    .ok_or(crate::optionable::Error {
                        missing_field: "git_tree_state",
                    })?,
            )?,
            git_version: crate::OptionableConvert::try_from_optioned(
                value
                    .git_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "git_version",
                    })?,
            )?,
            go_version: crate::OptionableConvert::try_from_optioned(
                value
                    .go_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "go_version",
                    })?,
            )?,
            major: crate::OptionableConvert::try_from_optioned(
                value
                    .major
                    .ok_or(crate::optionable::Error {
                        missing_field: "major",
                    })?,
            )?,
            minor: crate::OptionableConvert::try_from_optioned(
                value
                    .minor
                    .ok_or(crate::optionable::Error {
                        missing_field: "minor",
                    })?,
            )?,
            platform: crate::OptionableConvert::try_from_optioned(
                value
                    .platform
                    .ok_or(crate::optionable::Error {
                        missing_field: "platform",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: InfoAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.build_date {
            crate::OptionableConvert::merge(&mut self.build_date, other_value)?;
        }
        if let Some(other_value) = other.compiler {
            crate::OptionableConvert::merge(&mut self.compiler, other_value)?;
        }
        if let Some(other_value) = other.git_commit {
            crate::OptionableConvert::merge(&mut self.git_commit, other_value)?;
        }
        if let Some(other_value) = other.git_tree_state {
            crate::OptionableConvert::merge(&mut self.git_tree_state, other_value)?;
        }
        if let Some(other_value) = other.git_version {
            crate::OptionableConvert::merge(&mut self.git_version, other_value)?;
        }
        if let Some(other_value) = other.go_version {
            crate::OptionableConvert::merge(&mut self.go_version, other_value)?;
        }
        if let Some(other_value) = other.major {
            crate::OptionableConvert::merge(&mut self.major, other_value)?;
        }
        if let Some(other_value) = other.minor {
            crate::OptionableConvert::merge(&mut self.minor, other_value)?;
        }
        if let Some(other_value) = other.platform {
            crate::OptionableConvert::merge(&mut self.platform, other_value)?;
        }
        Ok(())
    }
}
