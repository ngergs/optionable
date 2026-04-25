#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EnvVarSource represents a source for the value of an EnvVar.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvVarSourceAc {
    /// Selects a key of a ConfigMap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_map_key_ref: Option<
        <::k8s_openapi027::api::core::v1::ConfigMapKeySelector as crate::Optionable>::Optioned,
    >,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels\['\<KEY\>'\]`, `metadata.annotations\['\<KEY\>'\]`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: Option<
        <::k8s_openapi027::api::core::v1::ObjectFieldSelector as crate::Optionable>::Optioned,
    >,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: Option<
        <::k8s_openapi027::api::core::v1::ResourceFieldSelector as crate::Optionable>::Optioned,
    >,
    /// Selects a key of a secret in the pod's namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key_ref: Option<
        <::k8s_openapi027::api::core::v1::SecretKeySelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EnvVarSource {
    type Optioned = EnvVarSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EnvVarSourceAc {
    type Optioned = EnvVarSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EnvVarSource {
    fn into_optioned(self) -> EnvVarSourceAc {
        EnvVarSourceAc {
            config_map_key_ref: crate::OptionableConvert::into_optioned(
                self.config_map_key_ref,
            ),
            field_ref: crate::OptionableConvert::into_optioned(self.field_ref),
            resource_field_ref: crate::OptionableConvert::into_optioned(
                self.resource_field_ref,
            ),
            secret_key_ref: crate::OptionableConvert::into_optioned(self.secret_key_ref),
        }
    }
    fn try_from_optioned(value: EnvVarSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config_map_key_ref: crate::OptionableConvert::try_from_optioned(
                value.config_map_key_ref,
            )?,
            field_ref: crate::OptionableConvert::try_from_optioned(value.field_ref)?,
            resource_field_ref: crate::OptionableConvert::try_from_optioned(
                value.resource_field_ref,
            )?,
            secret_key_ref: crate::OptionableConvert::try_from_optioned(
                value.secret_key_ref,
            )?,
        })
    }
    fn merge(&mut self, other: EnvVarSourceAc) -> Result<(), crate::Error> {
        if self.config_map_key_ref.is_none() {
            self.config_map_key_ref = crate::OptionableConvert::try_from_optioned(
                other.config_map_key_ref,
            )?;
        } else if let Some(self_value) = self.config_map_key_ref.as_mut()
            && let Some(other_value) = other.config_map_key_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.field_ref.is_none() {
            self.field_ref = crate::OptionableConvert::try_from_optioned(
                other.field_ref,
            )?;
        } else if let Some(self_value) = self.field_ref.as_mut()
            && let Some(other_value) = other.field_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_field_ref.is_none() {
            self.resource_field_ref = crate::OptionableConvert::try_from_optioned(
                other.resource_field_ref,
            )?;
        } else if let Some(self_value) = self.resource_field_ref.as_mut()
            && let Some(other_value) = other.resource_field_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.secret_key_ref.is_none() {
            self.secret_key_ref = crate::OptionableConvert::try_from_optioned(
                other.secret_key_ref,
            )?;
        } else if let Some(self_value) = self.secret_key_ref.as_mut()
            && let Some(other_value) = other.secret_key_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EnvVarSource>
for EnvVarSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EnvVarSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EnvVarSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EnvVarSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for EnvVarSourceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.config_map_key_ref,
            other.config_map_key_ref,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.field_ref, other.field_ref);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.resource_field_ref,
            other.resource_field_ref,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.secret_key_ref,
            other.secret_key_ref,
        );
    }
}
