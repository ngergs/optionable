#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceClass is a vendor- or admin-provided resource that contains device configuration and selectors. It can be referenced in the device requests of a claim to apply these presets. Cluster scoped.
///
/// This is an alpha type and requires enabling the DynamicResourceAllocation feature gate.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClassAc {
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
    /// Standard object metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// Spec defines what can be allocated and how to configure it.
    ///
    /// This is mutable. Consumers have to be prepared for classes changing at any time, either because they get updated or replaced. Claim allocations are done once based on whatever was set in classes at the time of allocation.
    ///
    /// Changing the spec automatically increments the metadata.generation number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::resource::v1alpha3::DeviceClassSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceClass {
    type Optioned = DeviceClassAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassAc {
    type Optioned = DeviceClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1alpha3::DeviceClass {
    fn into_optioned(self) -> DeviceClassAc {
        DeviceClassAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(value: DeviceClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceClassAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceClass>
for DeviceClassAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceClass,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::DeviceClass, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for DeviceClassAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for DeviceClassAc {
    type Ty = <k8s_openapi027::api::resource::v1alpha3::DeviceClass as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
impl k8s_openapi027::DeepMerge for DeviceClassAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.spec, other.spec);
    }
}
