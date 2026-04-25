#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodTemplate describes a template for creating copies of a predefined pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodTemplateAc {
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
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Template defines the pods that will be created from this pod template. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<
        <::k8s_openapi027::api::core::v1::PodTemplateSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodTemplate {
    type Optioned = PodTemplateAc;
}
#[automatically_derived]
impl crate::Optionable for PodTemplateAc {
    type Optioned = PodTemplateAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodTemplate {
    fn into_optioned(self) -> PodTemplateAc {
        PodTemplateAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(value: PodTemplateAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: PodTemplateAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if self.template.is_none() {
            self.template = crate::OptionableConvert::try_from_optioned(other.template)?;
        } else if let Some(self_value) = self.template.as_mut()
            && let Some(other_value) = other.template
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodTemplate>
for PodTemplateAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodTemplate) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodTemplate, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodTemplate,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for PodTemplateAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for PodTemplateAc {
    type Ty = <k8s_openapi027::api::core::v1::PodTemplate as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_podtemplateac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::PodTemplate>();
}
impl k8s_openapi027::DeepMerge for PodTemplateAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.template, other.template);
    }
}
