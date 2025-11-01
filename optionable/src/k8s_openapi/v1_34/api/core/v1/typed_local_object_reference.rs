pub struct TypedLocalObjectReferenceAc {
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TypedLocalObjectReference {
    type Optioned = TypedLocalObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for TypedLocalObjectReferenceAc {
    type Optioned = TypedLocalObjectReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::TypedLocalObjectReference {
    fn into_optioned(self) -> TypedLocalObjectReferenceAc {
        TypedLocalObjectReferenceAc {
            api_group: crate::OptionableConvert::into_optioned(self.api_group),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: TypedLocalObjectReferenceAc,
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
        })
    }
    fn merge(
        &mut self,
        other: TypedLocalObjectReferenceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_group, other.api_group)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
