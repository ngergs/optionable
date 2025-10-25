pub struct LeaseCandidateSpecOpt {
    pub binary_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub emulation_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub lease_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub ping_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub renew_time: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
    > as crate::Optionable>::Optioned,
    pub strategy: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec {
    type Optioned = LeaseCandidateSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateSpecOpt {
    type Optioned = LeaseCandidateSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec {
    fn into_optioned(self) -> LeaseCandidateSpecOpt {
        LeaseCandidateSpecOpt {
            binary_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.binary_version,
                ),
            ),
            emulation_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.emulation_version),
            lease_name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.lease_name,
                ),
            ),
            ping_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.ping_time),
            renew_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::into_optioned(self.renew_time),
            strategy: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.strategy,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: LeaseCandidateSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            binary_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .binary_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "binary_version",
                    })?,
            )?,
            emulation_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.emulation_version)?,
            lease_name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .lease_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "lease_name",
                    })?,
            )?,
            ping_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.ping_time)?,
            renew_time: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
            > as crate::OptionableConvert>::try_from_optioned(value.renew_time)?,
            strategy: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .strategy
                    .ok_or(crate::optionable::Error {
                        missing_field: "strategy",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: LeaseCandidateSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.binary_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.binary_version,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.emulation_version,
            other.emulation_version,
        )?;
        if let Some(other_value) = other.lease_name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.lease_name,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(&mut self.ping_time, other.ping_time)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime,
        > as crate::OptionableConvert>::merge(&mut self.renew_time, other.renew_time)?;
        if let Some(other_value) = other.strategy {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.strategy,
                other_value,
            )?;
        }
        Ok(())
    }
}
