#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StorageClassAc {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_volume_expansion: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_topologies: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::TopologySelectorTerm>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
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
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::storage::v1::StorageClass {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
impl crate::Optionable for StorageClassAc {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::storage::v1::StorageClass {
    fn into_optioned(self) -> StorageClassAc {
        StorageClassAc {
            api_version: Default::default(),
            kind: Default::default(),
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::storage::v1::StorageClass>
for StorageClassAc {
    fn from_optionable(value: k8s_openapi026::api::storage::v1::StorageClass) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::storage::v1::StorageClass, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::storage::v1::StorageClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for StorageClassAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for StorageClassAc {
    type Ty = <k8s_openapi026::api::storage::v1::StorageClass as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_storageclassac() {
    crate::testutil::roundtrip_test::<k8s_openapi026::api::storage::v1::StorageClass>();
}
