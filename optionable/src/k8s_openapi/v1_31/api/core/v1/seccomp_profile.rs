#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SeccompProfileAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localhost_profile: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SeccompProfile {
    type Optioned = SeccompProfileAc;
}
#[automatically_derived]
impl crate::Optionable for SeccompProfileAc {
    type Optioned = SeccompProfileAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SeccompProfile {
    fn into_optioned(self) -> SeccompProfileAc {
        SeccompProfileAc {
            localhost_profile: crate::OptionableConvert::into_optioned(
                self.localhost_profile,
            ),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(value: SeccompProfileAc) -> Result<Self, crate::Error> {
        Ok(Self {
            localhost_profile: crate::OptionableConvert::try_from_optioned(
                value.localhost_profile,
            )?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: SeccompProfileAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.localhost_profile,
            other.localhost_profile,
        )?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::SeccompProfile>
for SeccompProfileAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::SeccompProfile) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::SeccompProfile, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::SeccompProfile,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
