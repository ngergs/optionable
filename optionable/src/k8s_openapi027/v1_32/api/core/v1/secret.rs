#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Secret holds secret data of a certain type. The total bytes of the values in the Data field must be less than MaxSecretSize bytes.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SecretAc {
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
    /// Data contains the secret data. Each key must consist of alphanumeric characters, '-', '_' or '.'. The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here. Described in https://tools.ietf.org/html/rfc4648#section-4
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::ByteString as crate::Optionable>::Optioned,
        >,
    >,
    /// Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified). If not set to true, the field can be modified at any time. Defaulted to nil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// stringData allows specifying non-binary secret data in string form. It is provided as a write-only input field for convenience. All keys and values are merged into the data field on write, overwriting any existing values. The stringData field is never output when reading from the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_data: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    /// Used to facilitate programmatic handling of secret data. More info: https://kubernetes.io/docs/concepts/configuration/secret/#secret-types
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Secret {
    type Optioned = SecretAc;
}
#[automatically_derived]
impl crate::Optionable for SecretAc {
    type Optioned = SecretAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Secret {
    fn into_optioned(self) -> SecretAc {
        SecretAc {
            api_version: Default::default(),
            kind: Default::default(),
            data: crate::OptionableConvert::into_optioned(self.data),
            immutable: self.immutable,
            metadata: self.metadata,
            string_data: self.string_data,
            type_: self.type_,
        }
    }
    fn try_from_optioned(value: SecretAc) -> Result<Self, crate::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            immutable: value.immutable,
            metadata: value.metadata,
            string_data: value.string_data,
            type_: value.type_,
        })
    }
    fn merge(&mut self, other: SecretAc) -> Result<(), crate::Error> {
        if self.data.is_none() {
            self.data = crate::OptionableConvert::try_from_optioned(other.data)?;
        } else if let Some(self_value) = self.data.as_mut()
            && let Some(other_value) = other.data
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.immutable.is_none() {
            self.immutable = crate::OptionableConvert::try_from_optioned(
                other.immutable,
            )?;
        } else if let Some(self_value) = self.immutable.as_mut()
            && let Some(other_value) = other.immutable
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        self.metadata = other.metadata;
        if self.string_data.is_none() {
            self.string_data = crate::OptionableConvert::try_from_optioned(
                other.string_data,
            )?;
        } else if let Some(self_value) = self.string_data.as_mut()
            && let Some(other_value) = other.string_data
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.type_.is_none() {
            self.type_ = crate::OptionableConvert::try_from_optioned(other.type_)?;
        } else if let Some(self_value) = self.type_.as_mut()
            && let Some(other_value) = other.type_
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Secret> for SecretAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Secret) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Secret, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Secret,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for SecretAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for SecretAc {
    type Ty = <k8s_openapi027::api::core::v1::Secret as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_secretac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::core::v1::Secret>();
}
impl k8s_openapi027::DeepMerge for SecretAc {
    fn merge_from(&mut self, other: Self) {
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.data,
            other.data,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.immutable, other.immutable);
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::k8s_openapi::merge::merge_granular_option_wrapped(
            &mut self.string_data,
            other.string_data,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
