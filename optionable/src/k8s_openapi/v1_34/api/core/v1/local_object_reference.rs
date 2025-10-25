pub struct LocalObjectReferenceOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::local_object_reference::LocalObjectReference {
    type Optioned = LocalObjectReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for LocalObjectReferenceOpt {
    type Optioned = LocalObjectReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::local_object_reference::LocalObjectReference {
    fn into_optioned(self) -> LocalObjectReferenceOpt {
        LocalObjectReferenceOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: LocalObjectReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
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
        other: LocalObjectReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
