#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaseSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquire_time: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_identity: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_duration_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_transitions: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_holder: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renew_time: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: <Option<std::string::String> as crate::Optionable>::Optioned,
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
            holder_identity: crate::OptionableConvert::into_optioned(
                self.holder_identity,
            ),
            lease_duration_seconds: crate::OptionableConvert::into_optioned(
                self.lease_duration_seconds,
            ),
            lease_transitions: crate::OptionableConvert::into_optioned(
                self.lease_transitions,
            ),
            preferred_holder: crate::OptionableConvert::into_optioned(
                self.preferred_holder,
            ),
            renew_time: crate::OptionableConvert::into_optioned(self.renew_time),
            strategy: crate::OptionableConvert::into_optioned(self.strategy),
        }
    }
    fn try_from_optioned(value: LeaseSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            acquire_time: crate::OptionableConvert::try_from_optioned(
                value.acquire_time,
            )?,
            holder_identity: crate::OptionableConvert::try_from_optioned(
                value.holder_identity,
            )?,
            lease_duration_seconds: crate::OptionableConvert::try_from_optioned(
                value.lease_duration_seconds,
            )?,
            lease_transitions: crate::OptionableConvert::try_from_optioned(
                value.lease_transitions,
            )?,
            preferred_holder: crate::OptionableConvert::try_from_optioned(
                value.preferred_holder,
            )?,
            renew_time: crate::OptionableConvert::try_from_optioned(value.renew_time)?,
            strategy: crate::OptionableConvert::try_from_optioned(value.strategy)?,
        })
    }
    fn merge(&mut self, other: LeaseSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.acquire_time, other.acquire_time)?;
        crate::OptionableConvert::merge(
            &mut self.holder_identity,
            other.holder_identity,
        )?;
        crate::OptionableConvert::merge(
            &mut self.lease_duration_seconds,
            other.lease_duration_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.lease_transitions,
            other.lease_transitions,
        )?;
        crate::OptionableConvert::merge(
            &mut self.preferred_holder,
            other.preferred_holder,
        )?;
        crate::OptionableConvert::merge(&mut self.renew_time, other.renew_time)?;
        crate::OptionableConvert::merge(&mut self.strategy, other.strategy)?;
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
