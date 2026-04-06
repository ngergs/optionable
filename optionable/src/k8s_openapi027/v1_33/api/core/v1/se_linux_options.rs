#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SELinuxOptionsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SELinuxOptions {
    type Optioned = SELinuxOptionsAc;
}
#[automatically_derived]
impl crate::Optionable for SELinuxOptionsAc {
    type Optioned = SELinuxOptionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SELinuxOptions {
    fn into_optioned(self) -> SELinuxOptionsAc {
        SELinuxOptionsAc {
            level: crate::OptionableConvert::into_optioned(self.level),
            role: crate::OptionableConvert::into_optioned(self.role),
            type_: crate::OptionableConvert::into_optioned(self.type_),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(value: SELinuxOptionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            level: crate::OptionableConvert::try_from_optioned(value.level)?,
            role: crate::OptionableConvert::try_from_optioned(value.role)?,
            type_: crate::OptionableConvert::try_from_optioned(value.type_)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(&mut self, other: SELinuxOptionsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.level, other.level)?;
        crate::OptionableConvert::merge(&mut self.role, other.role)?;
        crate::OptionableConvert::merge(&mut self.type_, other.type_)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SELinuxOptions>
for SELinuxOptionsAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SELinuxOptions) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SELinuxOptions, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SELinuxOptions,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
