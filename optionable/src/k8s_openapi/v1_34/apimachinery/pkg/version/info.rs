pub struct InfoOpt {
    pub build_date: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub compiler: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub emulation_major: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub emulation_minor: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub git_commit: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub git_tree_state: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub git_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub go_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub major: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub min_compatibility_major: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub min_compatibility_minor: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub minor: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub platform: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::version::Info {
    type Optioned = InfoOpt;
}
#[automatically_derived]
impl crate::Optionable for InfoOpt {
    type Optioned = InfoOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::apimachinery::pkg::version::Info {
    fn into_optioned(self) -> InfoOpt {
        InfoOpt {
            build_date: Some(crate::OptionableConvert::into_optioned(self.build_date)),
            compiler: Some(crate::OptionableConvert::into_optioned(self.compiler)),
            emulation_major: crate::OptionableConvert::into_optioned(
                self.emulation_major,
            ),
            emulation_minor: crate::OptionableConvert::into_optioned(
                self.emulation_minor,
            ),
            git_commit: Some(crate::OptionableConvert::into_optioned(self.git_commit)),
            git_tree_state: Some(
                crate::OptionableConvert::into_optioned(self.git_tree_state),
            ),
            git_version: Some(crate::OptionableConvert::into_optioned(self.git_version)),
            go_version: Some(crate::OptionableConvert::into_optioned(self.go_version)),
            major: Some(crate::OptionableConvert::into_optioned(self.major)),
            min_compatibility_major: crate::OptionableConvert::into_optioned(
                self.min_compatibility_major,
            ),
            min_compatibility_minor: crate::OptionableConvert::into_optioned(
                self.min_compatibility_minor,
            ),
            minor: Some(crate::OptionableConvert::into_optioned(self.minor)),
            platform: Some(crate::OptionableConvert::into_optioned(self.platform)),
        }
    }
    fn try_from_optioned(value: InfoOpt) -> Result<Self, crate::optionable::Error> {
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
            emulation_major: crate::OptionableConvert::try_from_optioned(
                value.emulation_major,
            )?,
            emulation_minor: crate::OptionableConvert::try_from_optioned(
                value.emulation_minor,
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
            min_compatibility_major: crate::OptionableConvert::try_from_optioned(
                value.min_compatibility_major,
            )?,
            min_compatibility_minor: crate::OptionableConvert::try_from_optioned(
                value.min_compatibility_minor,
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
    fn merge(&mut self, other: InfoOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.build_date {
            crate::OptionableConvert::merge(&mut self.build_date, other_value)?;
        }
        if let Some(other_value) = other.compiler {
            crate::OptionableConvert::merge(&mut self.compiler, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.emulation_major,
            other.emulation_major,
        )?;
        crate::OptionableConvert::merge(
            &mut self.emulation_minor,
            other.emulation_minor,
        )?;
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
        crate::OptionableConvert::merge(
            &mut self.min_compatibility_major,
            other.min_compatibility_major,
        )?;
        crate::OptionableConvert::merge(
            &mut self.min_compatibility_minor,
            other.min_compatibility_minor,
        )?;
        if let Some(other_value) = other.minor {
            crate::OptionableConvert::merge(&mut self.minor, other_value)?;
        }
        if let Some(other_value) = other.platform {
            crate::OptionableConvert::merge(&mut self.platform, other_value)?;
        }
        Ok(())
    }
}
