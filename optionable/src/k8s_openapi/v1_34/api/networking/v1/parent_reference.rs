pub struct ParentReferenceOpt {
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::ParentReference {
    type Optioned = ParentReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for ParentReferenceOpt {
    type Optioned = ParentReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::ParentReference {
    fn into_optioned(self) -> ParentReferenceOpt {
        ParentReferenceOpt {
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.group),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
            resource: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.resource,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: ParentReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
            resource: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ParentReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        if let Some(other_value) = other.resource {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.resource,
                other_value,
            )?;
        }
        Ok(())
    }
}
