#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppArmorProfileAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localhost_profile: Option<std::string::String>,
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
        self.localhost_profile = other.localhost_profile;
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
