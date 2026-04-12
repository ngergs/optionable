#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// MutatingAdmissionPolicyBinding binds the MutatingAdmissionPolicy with parametrized resources. MutatingAdmissionPolicyBinding and the optional parameter resource together define how cluster administrators configure policies for clusters.
///
/// For a given admission request, each binding will cause its policy to be evaluated N times, where N is 1 for policies/bindings that don't use params, otherwise N is the number of parameters selected by the binding. Each evaluation is constrained by a \[runtime cost budget\](https://kubernetes.io/docs/reference/using-api/cel/#runtime-cost-budget).
///
/// Adding/removing policies, bindings, or params can not affect whether a given (policy, binding, param) combination is within its own CEL budget.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MutatingAdmissionPolicyBindingAc {
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
    /// Specification of the desired behavior of the MutatingAdmissionPolicyBinding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBindingSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding {
    type Optioned = MutatingAdmissionPolicyBindingAc;
}
#[automatically_derived]
impl crate::Optionable for MutatingAdmissionPolicyBindingAc {
    type Optioned = MutatingAdmissionPolicyBindingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding {
    fn into_optioned(self) -> MutatingAdmissionPolicyBindingAc {
        MutatingAdmissionPolicyBindingAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: MutatingAdmissionPolicyBindingAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(
        &mut self,
        other: MutatingAdmissionPolicyBindingAc,
    ) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if self.spec.is_none() {
            self.spec = crate::OptionableConvert::try_from_optioned(other.spec)?;
        } else if let Some(self_value) = self.spec.as_mut()
            && let Some(other_value) = other.spec
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding,
> for MutatingAdmissionPolicyBindingAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for MutatingAdmissionPolicyBindingAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for MutatingAdmissionPolicyBindingAc {
    type Ty = <k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_mutatingadmissionpolicybindingac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::admissionregistration::v1beta1::MutatingAdmissionPolicyBinding,
    >();
}
