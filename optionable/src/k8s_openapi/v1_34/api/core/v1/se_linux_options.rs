pub struct SELinuxOptionsOpt {
    pub level: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub role: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::se_linux_options::SELinuxOptions {
    type Optioned = SELinuxOptionsOpt;
}
#[automatically_derived]
impl crate::Optionable for SELinuxOptionsOpt {
    type Optioned = SELinuxOptionsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::se_linux_options::SELinuxOptions {
    fn into_optioned(self) -> SELinuxOptionsOpt {
        SELinuxOptionsOpt {
            level: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.level),
            role: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.role),
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.type_),
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: SELinuxOptionsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            level: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.level)?,
            role: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.role)?,
            type_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.type_)?,
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: SELinuxOptionsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.level, other.level)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.role, other.role)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.type_, other.type_)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
