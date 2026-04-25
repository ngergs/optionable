#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// VolumeAttributesClass represents a specification of mutable volume attributes defined by the CSI driver. The class can be specified during dynamic provisioning of PersistentVolumeClaims, and changed in the PersistentVolumeClaim spec after provisioning.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeAttributesClassAc {
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
    /// Name of the CSI driver This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: Option<std::string::String>,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// parameters hold volume attributes defined by the CSI driver. These values are opaque to the Kubernetes and are passed directly to the CSI driver. The underlying storage provider supports changing these attributes on an existing volume, however the parameters field itself is immutable. To invoke a volume update, a new VolumeAttributesClass should be created with new parameters, and the PersistentVolumeClaim should be updated to reference the new VolumeAttributesClass.
    ///
    /// This field is required and must contain at least one key/value pair. The keys cannot be empty, and the maximum number of parameters is 512, with a cumulative max size of 256K. If the CSI driver rejects invalid parameters, the target PersistentVolumeClaim will be set to an "Infeasible" state in the modifyVolumeStatus field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass {
    type Optioned = VolumeAttributesClassAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeAttributesClassAc {
    type Optioned = VolumeAttributesClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass {
    fn into_optioned(self) -> VolumeAttributesClassAc {
        VolumeAttributesClassAc {
            api_version: Default::default(),
            kind: Default::default(),
            driver_name: Some(self.driver_name),
            metadata: self.metadata,
            parameters: self.parameters,
        }
    }
    fn try_from_optioned(value: VolumeAttributesClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            driver_name: value
                .driver_name
                .ok_or(crate::Error {
                    missing_field: "driver_name",
                })?,
            metadata: value.metadata,
            parameters: value.parameters,
        })
    }
    fn merge(&mut self, other: VolumeAttributesClassAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver_name {
            self.driver_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.metadata = other.metadata;
        if self.parameters.is_none() {
            self.parameters = crate::OptionableConvert::try_from_optioned(
                other.parameters,
            )?;
        } else if let Some(self_value) = self.parameters.as_mut()
            && let Some(other_value) = other.parameters
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass,
> for VolumeAttributesClassAc {
    fn from_optionable(
        value: k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for VolumeAttributesClassAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for VolumeAttributesClassAc {
    type Ty = <k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_volumeattributesclassac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::storage::v1alpha1::VolumeAttributesClass,
    >();
}
impl k8s_openapi027::DeepMerge for VolumeAttributesClassAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(&mut self.driver_name, other.driver_name);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.parameters, other.parameters);
    }
}
