pub struct CrossVersionObjectReferenceOpt {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::autoscaling::v2::cross_version_object_reference::CrossVersionObjectReference {
    type Optioned = CrossVersionObjectReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for CrossVersionObjectReferenceOpt {
    type Optioned = CrossVersionObjectReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::autoscaling::v2::cross_version_object_reference::CrossVersionObjectReference {
    fn into_optioned(self) -> CrossVersionObjectReferenceOpt {
        CrossVersionObjectReferenceOpt {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.api_version),
            kind: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kind,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CrossVersionObjectReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.api_version)?,
            kind: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
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
        other: CrossVersionObjectReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.api_version, other.api_version)?;
        if let Some(other_value) = other.kind {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kind,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
