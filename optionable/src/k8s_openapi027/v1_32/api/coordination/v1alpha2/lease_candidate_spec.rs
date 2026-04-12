#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LeaseCandidateSpec is a specification of a Lease.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaseCandidateSpecAc {
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_version: Option<std::string::String>,
    /// EmulationVersion is the emulation version. It must be in a semver format without leading `v`. EmulationVersion must be less than or equal to BinaryVersion. This field is required when strategy is "OldestEmulationVersion"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emulation_version: Option<std::string::String>,
    /// LeaseName is the name of the lease for which this candidate is contending. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_name: Option<std::string::String>,
    /// PingTime is the last time that the server has requested the LeaseCandidate to renew. It is only done during leader election to check if any LeaseCandidates have become ineligible. When PingTime is updated, the LeaseCandidate will respond by updating RenewTime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    /// RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    /// Strategy is the strategy that coordinated leader election will use for picking the leader. If multiple candidates for the same Lease return different strategies, the strategy provided by the candidate with the latest BinaryVersion will be used. If there is still conflict, this is a user error and coordinated leader election will not operate the Lease until resolved. (Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.
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
            self.binary_version = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.emulation_version.is_none() {
            self.emulation_version = crate::OptionableConvert::try_from_optioned(
                other.emulation_version,
            )?;
        } else if let Some(self_value) = self.emulation_version.as_mut()
            && let Some(other_value) = other.emulation_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.lease_name {
            self.lease_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.ping_time.is_none() {
            self.ping_time = crate::OptionableConvert::try_from_optioned(
                other.ping_time,
            )?;
        } else if let Some(self_value) = self.ping_time.as_mut()
            && let Some(other_value) = other.ping_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.renew_time.is_none() {
            self.renew_time = crate::OptionableConvert::try_from_optioned(
                other.renew_time,
            )?;
        } else if let Some(self_value) = self.renew_time.as_mut()
            && let Some(other_value) = other.renew_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.strategy {
            self.strategy = crate::OptionableConvert::try_from_optioned(other_value)?;
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
