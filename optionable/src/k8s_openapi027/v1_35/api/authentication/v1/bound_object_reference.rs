#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// BoundObjectReference is a reference to an object that a token is bound to.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoundObjectReferenceAc {
    /// API version of the referent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// Kind of the referent. Valid kinds are 'Pod' and 'Secret'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Name of the referent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// UID of the referent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authentication::v1::BoundObjectReference {
    type Optioned = BoundObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for BoundObjectReferenceAc {
    type Optioned = BoundObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authentication::v1::BoundObjectReference {
    fn into_optioned(self) -> BoundObjectReferenceAc {
        BoundObjectReferenceAc {
            api_version: self.api_version,
            kind: self.kind,
            name: self.name,
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: BoundObjectReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            kind: value.kind,
            name: value.name,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: BoundObjectReferenceAc) -> Result<(), crate::Error> {
        self.api_version = other.api_version;
        self.kind = other.kind;
        self.name = other.name;
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authentication::v1::BoundObjectReference,
> for BoundObjectReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::authentication::v1::BoundObjectReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authentication::v1::BoundObjectReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authentication::v1::BoundObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
