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
    pub binary_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulation_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::coordination::v1alpha2::LeaseCandidateSpec {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateSpecAc {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::coordination::v1alpha2::LeaseCandidateSpec {
    fn into_optioned(self) -> LeaseCandidateSpecAc {
        LeaseCandidateSpecAc {
            binary_version: Some(self.binary_version),
            emulation_version: self.emulation_version,
            lease_name: Some(self.lease_name),
            ping_time: crate::OptionableConvert::into_optioned(self.ping_time),
            renew_time: crate::OptionableConvert::into_optioned(self.renew_time),
            strategy: Some(self.strategy),
        }
    }
    fn try_from_optioned(value: LeaseCandidateSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            binary_version: value
                .binary_version
                .ok_or(crate::Error {
                    missing_field: "binary_version",
                })?,
            emulation_version: value.emulation_version,
            lease_name: value
                .lease_name
                .ok_or(crate::Error {
                    missing_field: "lease_name",
                })?,
            ping_time: crate::OptionableConvert::try_from_optioned(value.ping_time)?,
            renew_time: crate::OptionableConvert::try_from_optioned(value.renew_time)?,
            strategy: value
                .strategy
                .ok_or(crate::Error {
                    missing_field: "strategy",
                })?,
        })
    }
    fn merge(&mut self, other: LeaseCandidateSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.binary_version {
            self.binary_version = other_value;
        }
        self.emulation_version = other.emulation_version;
        if let Some(other_value) = other.lease_name {
            self.lease_name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.ping_time, other.ping_time)?;
        crate::OptionableConvert::merge(&mut self.renew_time, other.renew_time)?;
        if let Some(other_value) = other.strategy {
            self.strategy = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::coordination::v1alpha2::LeaseCandidateSpec,
> for LeaseCandidateSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::coordination::v1alpha2::LeaseCandidateSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::coordination::v1alpha2::LeaseCandidateSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::coordination::v1alpha2::LeaseCandidateSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
