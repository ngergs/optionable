pub struct TypedObjectReferenceOpt {
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TypedObjectReference {
    type Optioned = TypedObjectReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for TypedObjectReferenceOpt {
    type Optioned = TypedObjectReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::TypedObjectReference {
    fn into_optioned(self) -> TypedObjectReferenceOpt {
        TypedObjectReferenceOpt {
            api_group: crate::OptionableConvert::into_optioned(self.api_group),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
        }
    }
    fn try_from_optioned(
        value: TypedObjectReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_group: crate::OptionableConvert::try_from_optioned(value.api_group)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
        })
    }
    fn merge(
        &mut self,
        other: TypedObjectReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
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
