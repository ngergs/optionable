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
            build_date: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.build_date,
                ),
            ),
            compiler: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.compiler,
                ),
            ),
            emulation_major: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.emulation_major),
            emulation_minor: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.emulation_minor),
            git_commit: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.git_commit,
                ),
            ),
            git_tree_state: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.git_tree_state,
                ),
            ),
            git_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.git_version,
                ),
            ),
            go_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.go_version,
                ),
            ),
            major: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.major,
                ),
            ),
            min_compatibility_major: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.min_compatibility_major),
            min_compatibility_minor: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.min_compatibility_minor),
            minor: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.minor,
                ),
            ),
            platform: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.platform,
                ),
            ),
        }
    }
    fn try_from_optioned(value: InfoOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            build_date: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .build_date
                    .ok_or(crate::optionable::Error {
                        missing_field: "build_date",
                    })?,
            )?,
            compiler: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .compiler
                    .ok_or(crate::optionable::Error {
                        missing_field: "compiler",
                    })?,
            )?,
            emulation_major: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.emulation_major)?,
            emulation_minor: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.emulation_minor)?,
            git_commit: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .git_commit
                    .ok_or(crate::optionable::Error {
                        missing_field: "git_commit",
                    })?,
            )?,
            git_tree_state: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .git_tree_state
                    .ok_or(crate::optionable::Error {
                        missing_field: "git_tree_state",
                    })?,
            )?,
            git_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .git_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "git_version",
                    })?,
            )?,
            go_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .go_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "go_version",
                    })?,
            )?,
            major: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .major
                    .ok_or(crate::optionable::Error {
                        missing_field: "major",
                    })?,
            )?,
            min_compatibility_major: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.min_compatibility_major,
            )?,
            min_compatibility_minor: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.min_compatibility_minor,
            )?,
            minor: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .minor
                    .ok_or(crate::optionable::Error {
                        missing_field: "minor",
                    })?,
            )?,
            platform: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.build_date,
                other_value,
            )?;
        }
        if let Some(other_value) = other.compiler {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.compiler,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.emulation_major,
            other.emulation_major,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.emulation_minor,
            other.emulation_minor,
        )?;
        if let Some(other_value) = other.git_commit {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.git_commit,
                other_value,
            )?;
        }
        if let Some(other_value) = other.git_tree_state {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.git_tree_state,
                other_value,
            )?;
        }
        if let Some(other_value) = other.git_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.git_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.go_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.go_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.major {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.major,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.min_compatibility_major,
            other.min_compatibility_major,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.min_compatibility_minor,
            other.min_compatibility_minor,
        )?;
        if let Some(other_value) = other.minor {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.minor,
                other_value,
            )?;
        }
        if let Some(other_value) = other.platform {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.platform,
                other_value,
            )?;
        }
        Ok(())
    }
}
