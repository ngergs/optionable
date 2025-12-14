#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIPersistentVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_expand_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_publish_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_expand_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_publish_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_stage_secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_attributes: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_handle: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource {
    type Optioned = CSIPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for CSIPersistentVolumeSourceAc {
    type Optioned = CSIPersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource {
    fn into_optioned(self) -> CSIPersistentVolumeSourceAc {
        CSIPersistentVolumeSourceAc {
            controller_expand_secret_ref: crate::OptionableConvert::into_optioned(
                self.controller_expand_secret_ref,
            ),
            controller_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.controller_publish_secret_ref,
            ),
            driver: Some(crate::OptionableConvert::into_optioned(self.driver)),
            fs_type: crate::OptionableConvert::into_optioned(self.fs_type),
            node_expand_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_expand_secret_ref,
            ),
            node_publish_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_publish_secret_ref,
            ),
            node_stage_secret_ref: crate::OptionableConvert::into_optioned(
                self.node_stage_secret_ref,
            ),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            volume_attributes: crate::OptionableConvert::into_optioned(
                self.volume_attributes,
            ),
            volume_handle: Some(
                crate::OptionableConvert::into_optioned(self.volume_handle),
            ),
        }
    }
    fn try_from_optioned(
        value: CSIPersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            controller_expand_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.controller_expand_secret_ref,
            )?,
            controller_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.controller_publish_secret_ref,
            )?,
            driver: crate::OptionableConvert::try_from_optioned(
                value
                    .driver
                    .ok_or(crate::Error {
                        missing_field: "driver",
                    })?,
            )?,
            fs_type: crate::OptionableConvert::try_from_optioned(value.fs_type)?,
            node_expand_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_expand_secret_ref,
            )?,
            node_publish_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_publish_secret_ref,
            )?,
            node_stage_secret_ref: crate::OptionableConvert::try_from_optioned(
                value.node_stage_secret_ref,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            volume_attributes: crate::OptionableConvert::try_from_optioned(
                value.volume_attributes,
            )?,
            volume_handle: crate::OptionableConvert::try_from_optioned(
                value
                    .volume_handle
                    .ok_or(crate::Error {
                        missing_field: "volume_handle",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSIPersistentVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.controller_expand_secret_ref,
            other.controller_expand_secret_ref,
        )?;
        crate::OptionableConvert::merge(
            &mut self.controller_publish_secret_ref,
            other.controller_publish_secret_ref,
        )?;
        if let Some(other_value) = other.driver {
            crate::OptionableConvert::merge(&mut self.driver, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.fs_type, other.fs_type)?;
        crate::OptionableConvert::merge(
            &mut self.node_expand_secret_ref,
            other.node_expand_secret_ref,
        )?;
        crate::OptionableConvert::merge(
            &mut self.node_publish_secret_ref,
            other.node_publish_secret_ref,
        )?;
        crate::OptionableConvert::merge(
            &mut self.node_stage_secret_ref,
            other.node_stage_secret_ref,
        )?;
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(
            &mut self.volume_attributes,
            other.volume_attributes,
        )?;
        if let Some(other_value) = other.volume_handle {
            crate::OptionableConvert::merge(&mut self.volume_handle, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::CSIPersistentVolumeSource>
for CSIPersistentVolumeSourceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::CSIPersistentVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::CSIPersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
