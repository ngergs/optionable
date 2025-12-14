#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaseCandidateSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulation_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_strategies: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::coordination::v1alpha1::LeaseCandidateSpec {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateSpecAc {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1alpha1::LeaseCandidateSpec {
    fn into_optioned(self) -> LeaseCandidateSpecAc {
        LeaseCandidateSpecAc {
            binary_version: crate::OptionableConvert::into_optioned(self.binary_version),
            emulation_version: crate::OptionableConvert::into_optioned(
                self.emulation_version,
            ),
            lease_name: Some(crate::OptionableConvert::into_optioned(self.lease_name)),
            ping_time: crate::OptionableConvert::into_optioned(self.ping_time),
            preferred_strategies: Some(
                crate::OptionableConvert::into_optioned(self.preferred_strategies),
            ),
            renew_time: crate::OptionableConvert::into_optioned(self.renew_time),
        }
    }
    fn try_from_optioned(value: LeaseCandidateSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            binary_version: crate::OptionableConvert::try_from_optioned(
                value.binary_version,
            )?,
            emulation_version: crate::OptionableConvert::try_from_optioned(
                value.emulation_version,
            )?,
            lease_name: crate::OptionableConvert::try_from_optioned(
                value
                    .lease_name
                    .ok_or(crate::Error {
                        missing_field: "lease_name",
                    })?,
            )?,
            ping_time: crate::OptionableConvert::try_from_optioned(value.ping_time)?,
            preferred_strategies: crate::OptionableConvert::try_from_optioned(
                value
                    .preferred_strategies
                    .ok_or(crate::Error {
                        missing_field: "preferred_strategies",
                    })?,
            )?,
            renew_time: crate::OptionableConvert::try_from_optioned(value.renew_time)?,
        })
    }
    fn merge(&mut self, other: LeaseCandidateSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.binary_version, other.binary_version)?;
        crate::OptionableConvert::merge(
            &mut self.emulation_version,
            other.emulation_version,
        )?;
        if let Some(other_value) = other.lease_name {
            crate::OptionableConvert::merge(&mut self.lease_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.ping_time, other.ping_time)?;
        if let Some(other_value) = other.preferred_strategies {
            crate::OptionableConvert::merge(
                &mut self.preferred_strategies,
                other_value,
            )?;
        }
        crate::OptionableConvert::merge(&mut self.renew_time, other.renew_time)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::coordination::v1alpha1::LeaseCandidateSpec,
> for LeaseCandidateSpecAc {
    fn from_optionable(
        value: ::k8s_openapi::api::coordination::v1alpha1::LeaseCandidateSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::coordination::v1alpha1::LeaseCandidateSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::coordination::v1alpha1::LeaseCandidateSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
