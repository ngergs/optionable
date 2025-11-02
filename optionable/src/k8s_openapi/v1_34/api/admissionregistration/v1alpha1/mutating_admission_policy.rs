#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    serde::Serialize,
    serde::Deserialize,
    kube::Resource
)]
#[resource(inherit = MutatingAdmissionPolicy)]
pub struct MutatingAdmissionPolicyAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicySpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
    type Optioned = MutatingAdmissionPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyAc {
    type Optioned = MutatingAdmissionPolicyAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
    fn into_optioned(self) -> MutatingAdmissionPolicyAc {
        MutatingAdmissionPolicyAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy;
