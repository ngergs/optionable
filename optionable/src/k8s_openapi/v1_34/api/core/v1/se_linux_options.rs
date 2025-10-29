pub struct SELinuxOptionsOpt {
    pub level: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub role: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SELinuxOptions {
    type Optioned = SELinuxOptionsOpt;
}
#[automatically_derived]
impl crate::Optionable for SELinuxOptionsOpt {
    type Optioned = SELinuxOptionsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SELinuxOptions {
    fn into_optioned(self) -> SELinuxOptionsOpt {
        SELinuxOptionsOpt {
            level: crate::OptionableConvert::into_optioned(self.level),
            role: crate::OptionableConvert::into_optioned(self.role),
            type_: crate::OptionableConvert::into_optioned(self.type_),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: SELinuxOptionsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            level: crate::OptionableConvert::try_from_optioned(value.level)?,
            role: crate::OptionableConvert::try_from_optioned(value.role)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: SELinuxOptionsOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.level, other.level)?;
        crate::OptionableConvert::merge(&mut self.role, other.role)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
