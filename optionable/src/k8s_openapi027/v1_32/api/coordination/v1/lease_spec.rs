#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LeaseSpec is a specification of a Lease.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaseSpecAc {
    /// acquireTime is a time when the current lease was acquired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquire_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    /// holderIdentity contains the identity of the holder of a current lease. If Coordinated Leader Election is used, the holder identity must be equal to the elected LeaseCandidate.metadata.name field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_identity: Option<std::string::String>,
    /// leaseDurationSeconds is a duration that candidates for a lease need to wait to force acquire it. This is measured against the time of last observed renewTime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_duration_seconds: Option<i32>,
    /// leaseTransitions is the number of transitions of a lease between holders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_transitions: Option<i32>,
    /// PreferredHolder signals to a lease holder that the lease has a more optimal holder and should be given up. This field can only be set if Strategy is also set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_holder: Option<std::string::String>,
    /// renewTime is a time when the current holder of a lease has last updated the lease.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime as crate::Optionable>::Optioned,
    >,
    /// Strategy indicates the strategy for picking the leader for coordinated leader election. If the field is not specified, there is no active coordination for this lease. (Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::coordination::v1::LeaseSpec {
    type Optioned = LeaseSpecAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseSpecAc {
    type Optioned = LeaseSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::coordination::v1::LeaseSpec {
    fn into_optioned(self) -> LeaseSpecAc {
        LeaseSpecAc {
            acquire_time: crate::OptionableConvert::into_optioned(self.acquire_time),
            holder_identity: self.holder_identity,
            lease_duration_seconds: self.lease_duration_seconds,
            lease_transitions: self.lease_transitions,
            preferred_holder: self.preferred_holder,
            renew_time: crate::OptionableConvert::into_optioned(self.renew_time),
            strategy: self.strategy,
        }
    }
    fn try_from_optioned(value: LeaseSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            acquire_time: crate::OptionableConvert::try_from_optioned(
                value.acquire_time,
            )?,
            holder_identity: value.holder_identity,
            lease_duration_seconds: value.lease_duration_seconds,
            lease_transitions: value.lease_transitions,
            preferred_holder: value.preferred_holder,
            renew_time: crate::OptionableConvert::try_from_optioned(value.renew_time)?,
            strategy: value.strategy,
        })
    }
    fn merge(&mut self, other: LeaseSpecAc) -> Result<(), crate::Error> {
        if self.acquire_time.is_none() {
            self.acquire_time = crate::OptionableConvert::try_from_optioned(
                other.acquire_time,
            )?;
        } else if let Some(self_value) = self.acquire_time.as_mut()
            && let Some(other_value) = other.acquire_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.holder_identity.is_none() {
            self.holder_identity = crate::OptionableConvert::try_from_optioned(
                other.holder_identity,
            )?;
        } else if let Some(self_value) = self.holder_identity.as_mut()
            && let Some(other_value) = other.holder_identity
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.lease_duration_seconds.is_none() {
            self.lease_duration_seconds = crate::OptionableConvert::try_from_optioned(
                other.lease_duration_seconds,
            )?;
        } else if let Some(self_value) = self.lease_duration_seconds.as_mut()
            && let Some(other_value) = other.lease_duration_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.lease_transitions.is_none() {
            self.lease_transitions = crate::OptionableConvert::try_from_optioned(
                other.lease_transitions,
            )?;
        } else if let Some(self_value) = self.lease_transitions.as_mut()
            && let Some(other_value) = other.lease_transitions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.preferred_holder.is_none() {
            self.preferred_holder = crate::OptionableConvert::try_from_optioned(
                other.preferred_holder,
            )?;
        } else if let Some(self_value) = self.preferred_holder.as_mut()
            && let Some(other_value) = other.preferred_holder
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
        if self.strategy.is_none() {
            self.strategy = crate::OptionableConvert::try_from_optioned(other.strategy)?;
        } else if let Some(self_value) = self.strategy.as_mut()
            && let Some(other_value) = other.strategy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::coordination::v1::LeaseSpec>
for LeaseSpecAc {
    fn from_optionable(value: k8s_openapi027::api::coordination::v1::LeaseSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::coordination::v1::LeaseSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::coordination::v1::LeaseSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
