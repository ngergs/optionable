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
        if self.level.is_none() {
            self.level = crate::OptionableConvert::try_from_optioned(other.level)?;
        } else if let Some(self_value) = self.level.as_mut()
            && let Some(other_value) = other.level
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.role.is_none() {
            self.role = crate::OptionableConvert::try_from_optioned(other.role)?;
        } else if let Some(self_value) = self.role.as_mut()
            && let Some(other_value) = other.role
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else if let Some(self_value) = self.type_.as_mut()
            && let Some(other_value) = other.type_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.user.is_none() {
            self.user = crate::OptionableConvert::try_from_optioned(other.user)?;
        } else if let Some(self_value) = self.user.as_mut()
            && let Some(other_value) = other.user
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
