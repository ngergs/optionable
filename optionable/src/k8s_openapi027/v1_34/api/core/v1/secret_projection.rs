#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Adapts a secret into a projected volume.
///
/// The contents of the target Secret's Data field will be presented in a projected volume as files using the keys in the Data field as the file names. Note that this is identical to a secret volume source without the default mode.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretProjectionAc {
    /// items if unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::KeyToPath as crate::Optionable>::Optioned,
        >,
    >,
    /// Name of the referent. This field is effectively required, but due to backwards compatibility is allowed to be empty. Instances of this type with an empty value here are almost certainly wrong. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// optional field specify whether the Secret or its key must be defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::SecretProjection {
    type Optioned = SecretProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for SecretProjectionAc {
    type Optioned = SecretProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::SecretProjection {
    fn into_optioned(self) -> SecretProjectionAc {
        SecretProjectionAc {
            items: crate::OptionableConvert::into_optioned(self.items),
            name: Some(self.name),
            optional: self.optional,
        }
    }
    fn try_from_optioned(value: SecretProjectionAc) -> Result<Self, crate::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(value.items)?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            optional: value.optional,
        })
    }
    fn merge(&mut self, other: SecretProjectionAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.items, other.items)?;
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        self.optional = other.optional;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::SecretProjection>
for SecretProjectionAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::SecretProjection) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::SecretProjection, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::SecretProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
