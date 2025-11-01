pub struct LeaseCandidateOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::coordination::v1beta1::LeaseCandidateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::coordination::v1beta1::LeaseCandidate {
    type Optioned = LeaseCandidateOpt;
}
#[automatically_derived]
impl crate::Optionable for LeaseCandidateOpt {
    type Optioned = LeaseCandidateOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::coordination::v1beta1::LeaseCandidate {
    fn into_optioned(self) -> LeaseCandidateOpt {
        LeaseCandidateOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: LeaseCandidateOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: LeaseCandidateOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
