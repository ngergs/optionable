#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct StorageClassAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_topologies: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::TopologySelectorTerm>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioner: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_binding_mode: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<StorageClassAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::StorageClass {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
impl crate::Optionable for StorageClassAc {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::StorageClass {
    fn into_optioned(self) -> StorageClassAc {
        StorageClassAc {
            allow_volume_expansion: crate::OptionableConvert::into_optioned(
                self.allow_volume_expansion,
            ),
            allowed_topologies: crate::OptionableConvert::into_optioned(
                self.allowed_topologies,
            ),
            metadata: self.metadata,
            mount_options: crate::OptionableConvert::into_optioned(self.mount_options),
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
            provisioner: Some(crate::OptionableConvert::into_optioned(self.provisioner)),
            reclaim_policy: crate::OptionableConvert::into_optioned(self.reclaim_policy),
            volume_binding_mode: crate::OptionableConvert::into_optioned(
                self.volume_binding_mode,
            ),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: StorageClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allow_volume_expansion: crate::OptionableConvert::try_from_optioned(
                value.allow_volume_expansion,
            )?,
            allowed_topologies: crate::OptionableConvert::try_from_optioned(
                value.allowed_topologies,
            )?,
            metadata: value.metadata,
            mount_options: crate::OptionableConvert::try_from_optioned(
                value.mount_options,
            )?,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
            provisioner: crate::OptionableConvert::try_from_optioned(
                value
                    .provisioner
                    .ok_or(crate::Error {
                        missing_field: "provisioner",
                    })?,
            )?,
            reclaim_policy: crate::OptionableConvert::try_from_optioned(
                value.reclaim_policy,
            )?,
            volume_binding_mode: crate::OptionableConvert::try_from_optioned(
                value.volume_binding_mode,
            )?,
        })
    }
    fn merge(&mut self, other: StorageClassAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.allow_volume_expansion,
            other.allow_volume_expansion,
        )?;
        crate::OptionableConvert::merge(
            &mut self.allowed_topologies,
            other.allowed_topologies,
        )?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.mount_options, other.mount_options)?;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        if let Some(other_value) = other.provisioner {
            crate::OptionableConvert::merge(&mut self.provisioner, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.reclaim_policy, other.reclaim_policy)?;
        crate::OptionableConvert::merge(
            &mut self.volume_binding_mode,
            other.volume_binding_mode,
        )?;
        Ok(())
    }
}
impl k8s_openapi::Resource for StorageClassAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for StorageClassAc {
    type Ty = <::k8s_openapi::api::storage::v1::StorageClass as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
