pub struct LeaseSpecOpt {
    pub acquire_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub holder_identity: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub lease_duration_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub lease_transitions: <Option<i32> as crate::Optionable>::Optioned,
    pub preferred_holder: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub renew_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub strategy: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::coordination::v1::lease_spec::LeaseSpec {
    type Optioned = LeaseSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for LeaseSpecOpt {
    type Optioned = LeaseSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1::lease_spec::LeaseSpec {
    fn into_optioned(self) -> LeaseSpecOpt {
        LeaseSpecOpt {
            acquire_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.acquire_time),
            holder_identity: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.holder_identity),
            lease_duration_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.lease_duration_seconds),
            lease_transitions: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.lease_transitions),
            preferred_holder: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.preferred_holder),
            renew_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.renew_time),
            strategy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.strategy),
        }
    }
    fn try_from_optioned(value: LeaseSpecOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            acquire_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.acquire_time)?,
            holder_identity: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.holder_identity)?,
            lease_duration_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(
                value.lease_duration_seconds,
            )?,
            lease_transitions: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.lease_transitions)?,
            preferred_holder: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.preferred_holder)?,
            renew_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.renew_time)?,
            strategy: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.strategy)?,
        })
    }
    fn merge(&mut self, other: LeaseSpecOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(
            &mut self.acquire_time,
            other.acquire_time,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.holder_identity,
            other.holder_identity,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.lease_duration_seconds,
            other.lease_duration_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.lease_transitions,
            other.lease_transitions,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.preferred_holder,
            other.preferred_holder,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(&mut self.renew_time, other.renew_time)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.strategy, other.strategy)?;
        Ok(())
    }
}
