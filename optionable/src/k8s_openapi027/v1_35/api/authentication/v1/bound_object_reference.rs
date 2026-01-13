#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoundObjectReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
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
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: crate::OptionableConvert::into_optioned(self.kind),
            name: crate::OptionableConvert::into_optioned(self.name),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(value: BoundObjectReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(&mut self, other: BoundObjectReferenceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
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
