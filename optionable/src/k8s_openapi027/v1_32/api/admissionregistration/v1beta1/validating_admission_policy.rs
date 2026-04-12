#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatingAdmissionPolicyAc {
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
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Specification of the desired behavior of the ValidatingAdmissionPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicySpec as crate::Optionable>::Optioned,
    >,
    /// The status of the ValidatingAdmissionPolicy, including warnings that are useful to determine if the policy behaves in the expected way. Populated by the system. Read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicyStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy {
    type Optioned = ValidatingAdmissionPolicyAc;
}
#[automatically_derived]
impl crate::Optionable for ValidatingAdmissionPolicyAc {
    type Optioned = ValidatingAdmissionPolicyAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy {
    fn into_optioned(self) -> ValidatingAdmissionPolicyAc {
        ValidatingAdmissionPolicyAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
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
        if self.spec.is_none() {
            self.spec = crate::OptionableConvert::try_from_optioned(other.spec)?;
        } else if let Some(self_value) = self.spec.as_mut()
            && let Some(other_value) = other.spec
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.status.is_none() {
            self.status = crate::OptionableConvert::try_from_optioned(other.status)?;
        } else if let Some(self_value) = self.status.as_mut()
            && let Some(other_value) = other.status
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy,
> for ValidatingAdmissionPolicyAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for ValidatingAdmissionPolicyAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for ValidatingAdmissionPolicyAc {
    type Ty = <k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_validatingadmissionpolicyac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::admissionregistration::v1beta1::ValidatingAdmissionPolicy,
    >();
}
