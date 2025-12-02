#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct OwnerReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_owner_deletion: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference {
    type Optioned = OwnerReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for OwnerReferenceAc {
    type Optioned = OwnerReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference {
    fn into_optioned(self) -> OwnerReferenceAc {
        OwnerReferenceAc {
            api_version: Some(crate::OptionableConvert::into_optioned(self.api_version)),
            block_owner_deletion: crate::OptionableConvert::into_optioned(
                self.block_owner_deletion,
            ),
            controller: crate::OptionableConvert::into_optioned(self.controller),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            uid: Some(crate::OptionableConvert::into_optioned(self.uid)),
        }
    }
    fn try_from_optioned(value: OwnerReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(
                value
                    .api_version
                    .ok_or(crate::Error {
                        missing_field: "api_version",
                    })?,
            )?,
            block_owner_deletion: crate::OptionableConvert::try_from_optioned(
                value.block_owner_deletion,
            )?,
            controller: crate::OptionableConvert::try_from_optioned(value.controller)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(
                value
                    .uid
                    .ok_or(crate::Error {
                        missing_field: "uid",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: OwnerReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.api_version {
            crate::OptionableConvert::merge(&mut self.api_version, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.block_owner_deletion,
            other.block_owner_deletion,
        )?;
        crate::OptionableConvert::merge(&mut self.controller, other.controller)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.uid {
            crate::OptionableConvert::merge(&mut self.uid, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
> for OwnerReferenceAc {
    fn from_optionable(
        value: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
