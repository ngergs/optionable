#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ParentReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1alpha1::ParentReference {
    type Optioned = ParentReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for ParentReferenceAc {
    type Optioned = ParentReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1alpha1::ParentReference {
    fn into_optioned(self) -> ParentReferenceAc {
        ParentReferenceAc {
            group: crate::OptionableConvert::into_optioned(self.group),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
        }
    }
    fn try_from_optioned(value: ParentReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ParentReferenceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        Ok(())
    }
}
