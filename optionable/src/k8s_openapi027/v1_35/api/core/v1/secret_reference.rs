#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SecretReference represents a Secret Reference. It has enough information to retrieve secret in any namespace
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretReferenceAc {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SecretReference {
    type Optioned = SecretReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for SecretReferenceAc {
    type Optioned = SecretReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SecretReference {
    fn into_optioned(self) -> SecretReferenceAc {
        SecretReferenceAc {
            name: self.name,
            namespace: self.namespace,
        }
    }
    fn try_from_optioned(value: SecretReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            namespace: value.namespace,
        })
    }
    fn merge(&mut self, other: SecretReferenceAc) -> Result<(), crate::Error> {
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else if let Some(self_value) = self.name.as_mut()
            && let Some(other_value) = other.name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.namespace.is_none() {
            self.namespace = crate::OptionableConvert::try_from_optioned(
                other.namespace,
            )?;
        } else if let Some(self_value) = self.namespace.as_mut()
            && let Some(other_value) = other.namespace
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SecretReference>
for SecretReferenceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SecretReference) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SecretReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SecretReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
