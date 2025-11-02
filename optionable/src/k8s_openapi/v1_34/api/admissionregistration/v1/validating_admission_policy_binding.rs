#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyBindingAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<ValidatingAdmissionPolicyBindingAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBinding {
    type Optioned = ValidatingAdmissionPolicyBindingAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyBindingAc {
    type Optioned = ValidatingAdmissionPolicyBindingAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyBinding {
    fn into_optioned(self) -> ValidatingAdmissionPolicyBindingAc {
        ValidatingAdmissionPolicyBindingAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicyBindingAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: ValidatingAdmissionPolicyBindingAc,
    ) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for ValidatingAdmissionPolicyBindingAc {
    const API_VERSION: &'static str = "admissionregistration.k8s.io/v1";
    const GROUP: &'static str = "admissionregistration.k8s.io";
    const KIND: &'static str = "ValidatingAdmissionPolicyBinding";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "validatingadmissionpolicybindings";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for ValidatingAdmissionPolicyBindingAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
