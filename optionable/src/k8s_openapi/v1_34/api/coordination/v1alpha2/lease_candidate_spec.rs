pub struct LeaseCandidateSpecAc {
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
for ::k8s_openapi::api::coordination::v1alpha2::LeaseCandidateSpec {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateSpecAc {
    type Optioned = LeaseCandidateSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1alpha2::LeaseCandidateSpec {
    fn into_optioned(self) -> LeaseCandidateSpecAc {
        LeaseCandidateSpecAc {
            binary_version: Some(
                crate::OptionableConvert::into_optioned(self.binary_version),
            ),
            emulation_version: crate::OptionableConvert::into_optioned(
                self.emulation_version,
            ),
            lease_name: Some(crate::OptionableConvert::into_optioned(self.lease_name)),
            ping_time: crate::OptionableConvert::into_optioned(self.ping_time),
            renew_time: crate::OptionableConvert::into_optioned(self.renew_time),
            strategy: Some(crate::OptionableConvert::into_optioned(self.strategy)),
        }
    }
    fn try_from_optioned(
        value: LeaseCandidateSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            binary_version: crate::OptionableConvert::try_from_optioned(
                value
                    .binary_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "binary_version",
                    })?,
            )?,
            emulation_version: crate::OptionableConvert::try_from_optioned(
                value.emulation_version,
            )?,
            lease_name: crate::OptionableConvert::try_from_optioned(
                value
                    .lease_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "lease_name",
                    })?,
            )?,
            ping_time: crate::OptionableConvert::try_from_optioned(value.ping_time)?,
            renew_time: crate::OptionableConvert::try_from_optioned(value.renew_time)?,
            strategy: crate::OptionableConvert::try_from_optioned(
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
        other: LeaseCandidateSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.binary_version {
            crate::OptionableConvert::merge(&mut self.binary_version, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.emulation_version,
            other.emulation_version,
        )?;
        if let Some(other_value) = other.lease_name {
            crate::OptionableConvert::merge(&mut self.lease_name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.ping_time, other.ping_time)?;
        crate::OptionableConvert::merge(&mut self.renew_time, other.renew_time)?;
        if let Some(other_value) = other.strategy {
            crate::OptionableConvert::merge(&mut self.strategy, other_value)?;
        }
        Ok(())
    }
}
