#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SELinuxOptions are the labels to be applied to the container
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SELinuxOptionsAc {
    /// Level is SELinux level label that applies to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<std::string::String>,
    /// Role is a SELinux role label that applies to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<std::string::String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
    /// User is a SELinux user label that applies to the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
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
            level: self.level,
            role: self.role,
            type_: self.type_,
            user: self.user,
        }
    }
    fn try_from_optioned(value: SELinuxOptionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            level: value.level,
            role: value.role,
            type_: value.type_,
            user: value.user,
        })
    }
    fn merge(&mut self, other: SELinuxOptionsAc) -> Result<(), crate::Error> {
        self.level = other.level;
        self.role = other.role;
        self.type_ = other.type_;
        self.user = other.user;
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
