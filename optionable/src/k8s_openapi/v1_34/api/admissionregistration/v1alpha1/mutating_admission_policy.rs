pub struct MutatingAdmissionPolicyOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicySpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
    type Optioned = MutatingAdmissionPolicyOpt;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyOpt {
    type Optioned = MutatingAdmissionPolicyOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
    fn into_optioned(self) -> MutatingAdmissionPolicyOpt {
        MutatingAdmissionPolicyOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyOpt,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
