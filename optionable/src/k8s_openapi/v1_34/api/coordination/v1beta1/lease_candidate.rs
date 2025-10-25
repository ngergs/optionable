pub struct LeaseCandidateOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: <Option<
        ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::coordination::v1beta1::lease_candidate::LeaseCandidate {
    type Optioned = LeaseCandidateOpt;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateOpt {
    type Optioned = LeaseCandidateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1beta1::lease_candidate::LeaseCandidate {
    fn into_optioned(self) -> LeaseCandidateOpt {
        LeaseCandidateOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: <Option<
                ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec,
            > as crate::OptionableConvert>::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: LeaseCandidateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <Option<
                ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: LeaseCandidateOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec,
        > as crate::OptionableConvert>::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
