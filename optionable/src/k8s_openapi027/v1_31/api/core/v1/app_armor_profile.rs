#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AppArmorProfile defines a pod or container's AppArmor settings.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppArmorProfileAc {
    /// localhostProfile indicates a profile loaded on the node that should be used. The profile must be preconfigured on the node to work. Must match the loaded name of the profile. Must be set if and only if type is "Localhost".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<std::string::String>,
    /// type indicates which kind of AppArmor profile will be applied. Valid options are:
    ///   Localhost - a profile pre-loaded on the node.
    ///   RuntimeDefault - the container runtime's default profile.
    ///   Unconfined - no AppArmor enforcement.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::AppArmorProfile {
    type Optioned = AppArmorProfileAc;
}
#[automatically_derived]
impl crate::Optionable for AppArmorProfileAc {
    type Optioned = AppArmorProfileAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::AppArmorProfile {
    fn into_optioned(self) -> AppArmorProfileAc {
        AppArmorProfileAc {
            localhost_profile: self.localhost_profile,
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: AppArmorProfileAc) -> Result<Self, crate::Error> {
        Ok(Self {
            localhost_profile: value.localhost_profile,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: AppArmorProfileAc) -> Result<(), crate::Error> {
        if other.localhost_profile.is_some() {
            self.localhost_profile = other.localhost_profile;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::AppArmorProfile>
for AppArmorProfileAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::AppArmorProfile) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::AppArmorProfile, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::AppArmorProfile,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
