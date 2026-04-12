#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SeccompProfile defines a pod/container's seccomp profile settings. Only one profile source may be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SeccompProfileAc {
    /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must be set if type is "Localhost". Must NOT be set for any other type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<std::string::String>,
    /// type indicates which kind of seccomp profile will be applied. Valid options are:
    ///
    /// Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SeccompProfile {
    type Optioned = SeccompProfileAc;
}
#[automatically_derived]
impl crate::Optionable for SeccompProfileAc {
    type Optioned = SeccompProfileAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SeccompProfile {
    fn into_optioned(self) -> SeccompProfileAc {
        SeccompProfileAc {
            localhost_profile: self.localhost_profile,
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: SeccompProfileAc) -> Result<Self, crate::Error> {
        Ok(Self {
            localhost_profile: value.localhost_profile,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: SeccompProfileAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SeccompProfile>
for SeccompProfileAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SeccompProfile) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SeccompProfile, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SeccompProfile,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
