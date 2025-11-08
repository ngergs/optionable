#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct TypedObjectReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TypedObjectReference {
    type Optioned = TypedObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for TypedObjectReferenceAc {
    type Optioned = TypedObjectReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::TypedObjectReference {
    fn into_optioned(self) -> TypedObjectReferenceAc {
        TypedObjectReferenceAc {
            api_group: crate::OptionableConvert::into_optioned(self.api_group),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
        }
    }
    fn try_from_optioned(value: TypedObjectReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_group: crate::OptionableConvert::try_from_optioned(value.api_group)?,
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
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
        })
    }
    fn merge(&mut self, other: TypedObjectReferenceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_group, other.api_group)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        Ok(())
    }
}
