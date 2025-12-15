#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutatingAdmissionPolicyAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy {
    fn into_optioned(self) -> MutatingAdmissionPolicyAc {
        MutatingAdmissionPolicyAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: MutatingAdmissionPolicyAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy,
> for MutatingAdmissionPolicyAc {
    fn from_optionable(
        value: ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi::Resource for MutatingAdmissionPolicyAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for MutatingAdmissionPolicyAc {
    type Ty = <::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_mutatingadmissionpolicyac() {
    crate::testutil::roundtrip_test::<
        ::k8s_openapi::api::admissionregistration::v1alpha1::MutatingAdmissionPolicy,
    >();
}
