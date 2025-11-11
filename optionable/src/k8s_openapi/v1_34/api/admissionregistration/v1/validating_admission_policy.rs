#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ValidatingAdmissionPolicyAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicySpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicyStatus,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        deserialize_with = "crate::k8s_openapi::deserialize_api_envelope"
    )]
    pub phantom: std::marker::PhantomData<ValidatingAdmissionPolicyAc>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy {
    type Optioned = ValidatingAdmissionPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyAc {
    type Optioned = ValidatingAdmissionPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy {
    fn into_optioned(self) -> ValidatingAdmissionPolicyAc {
        ValidatingAdmissionPolicyAc {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(
        value: ValidatingAdmissionPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: ValidatingAdmissionPolicyAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for ValidatingAdmissionPolicyAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for ValidatingAdmissionPolicyAc {
    type Ty = <::k8s_openapi::api::admissionregistration::v1::ValidatingAdmissionPolicy as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
