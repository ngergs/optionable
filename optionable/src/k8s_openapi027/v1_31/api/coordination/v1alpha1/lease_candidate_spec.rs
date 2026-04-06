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
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`. This field is required when strategy is "OldestEmulationVersion"
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
    /// PreferredStrategies indicates the list of strategies for picking the leader for coordinated leader election. The list is ordered, and the first strategy supersedes all other strategies. The list is used by coordinated leader election to make a decision about the final election strategy. This follows as - If all clients have strategy X as the first element in this list, strategy X will be used. - If a candidate has strategy \[X\] and another candidate has strategy \[Y, X\], Y supersedes X and strategy Y
    ///   will be used.
    /// - If a candidate has strategy \[X, Y\] and another candidate has strategy \[Y, X\], this is a user error and leader
    ///   election will not operate the Lease until resolved.
    /// (Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_strategies: Option<std::vec::Vec<std::string::String>>,
    /// RenewTime is the time that the LeaseCandidate was last updated. Any time a Lease needs to do leader election, the PingTime field is updated to signal to the LeaseCandidate that they should update the RenewTime. Old LeaseCandidate objects are also garbage collected if it has been hours since the last renew. The PingTime field is updated regularly to prevent garbage collection for still active LeaseCandidates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateSpecAc {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec {
    fn into_optioned(self) -> LeaseCandidateSpecAc {
        LeaseCandidateSpecAc {
            binary_version: self.binary_version,
            emulation_version: self.emulation_version,
            lease_name: Some(self.lease_name),
            ping_time: crate::OptionableConvert::into_optioned(self.ping_time),
            preferred_strategies: Some(self.preferred_strategies),
            renew_time: crate::OptionableConvert::into_optioned(self.renew_time),
        }
    }
    fn try_from_optioned(value: LeaseCandidateSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            binary_version: value.binary_version,
            emulation_version: value.emulation_version,
            lease_name: value
                .lease_name
                .ok_or(crate::Error {
                    missing_field: "lease_name",
                })?,
            ping_time: crate::OptionableConvert::try_from_optioned(value.ping_time)?,
            preferred_strategies: value
                .preferred_strategies
                .ok_or(crate::Error {
                    missing_field: "preferred_strategies",
                })?,
            renew_time: crate::OptionableConvert::try_from_optioned(value.renew_time)?,
        })
    }
    fn merge(&mut self, other: LeaseCandidateSpecAc) -> Result<(), crate::Error> {
        self.binary_version = other.binary_version;
        self.emulation_version = other.emulation_version;
        if let Some(other_value) = other.lease_name {
            self.lease_name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.ping_time, other.ping_time)?;
        if let Some(other_value) = other.preferred_strategies {
            self.preferred_strategies = other_value;
        }
        crate::OptionableConvert::merge(&mut self.renew_time, other.renew_time)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec,
> for LeaseCandidateSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::coordination::v1alpha1::LeaseCandidateSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
