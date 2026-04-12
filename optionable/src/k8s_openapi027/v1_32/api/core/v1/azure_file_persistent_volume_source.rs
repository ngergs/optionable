#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AzureFilePersistentVolumeSourceAc {
    /// readOnly defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// secretName is the name of secret that contains Azure Storage Account Name and Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<std::string::String>,
    /// secretNamespace is the namespace of the secret that contains Azure Storage Account Name and Key default is the same as the Pod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_namespace: Option<std::string::String>,
    /// shareName is the azure Share Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource {
    type Optioned = AzureFilePersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for AzureFilePersistentVolumeSourceAc {
    type Optioned = AzureFilePersistentVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource {
    fn into_optioned(self) -> AzureFilePersistentVolumeSourceAc {
        AzureFilePersistentVolumeSourceAc {
            read_only: self.read_only,
            secret_name: Some(self.secret_name),
            secret_namespace: self.secret_namespace,
            share_name: Some(self.share_name),
        }
    }
    fn try_from_optioned(
        value: AzureFilePersistentVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            read_only: value.read_only,
            secret_name: value
                .secret_name
                .ok_or(crate::Error {
                    missing_field: "secret_name",
                })?,
            secret_namespace: value.secret_namespace,
            share_name: value
                .share_name
                .ok_or(crate::Error {
                    missing_field: "share_name",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: AzureFilePersistentVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if self.read_only.is_none() {
            self.read_only = crate::OptionableConvert::try_from_optioned(
                other.read_only,
            )?;
        } else if let Some(self_value) = self.read_only.as_mut()
            && let Some(other_value) = other.read_only
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.secret_name {
            self.secret_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.secret_namespace.is_none() {
            self.secret_namespace = crate::OptionableConvert::try_from_optioned(
                other.secret_namespace,
            )?;
        } else if let Some(self_value) = self.secret_namespace.as_mut()
            && let Some(other_value) = other.secret_namespace
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.share_name {
            self.share_name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource,
> for AzureFilePersistentVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::AzureFilePersistentVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
