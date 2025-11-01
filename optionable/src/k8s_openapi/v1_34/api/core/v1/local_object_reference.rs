pub struct LocalObjectReferenceAc {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LocalObjectReference {
    type Optioned = LocalObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for LocalObjectReferenceAc {
    type Optioned = LocalObjectReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LocalObjectReference {
    fn into_optioned(self) -> LocalObjectReferenceAc {
        LocalObjectReferenceAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: LocalObjectReferenceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
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
        other: LocalObjectReferenceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
