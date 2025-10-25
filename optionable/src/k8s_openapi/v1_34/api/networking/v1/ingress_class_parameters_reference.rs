pub struct IngressClassParametersReferenceOpt {
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub scope: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::ingress_class_parameters_reference::IngressClassParametersReference {
    type Optioned = IngressClassParametersReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressClassParametersReferenceOpt {
    type Optioned = IngressClassParametersReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ingress_class_parameters_reference::IngressClassParametersReference {
    fn into_optioned(self) -> IngressClassParametersReferenceOpt {
        IngressClassParametersReferenceOpt {
            api_group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.api_group),
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
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
            scope: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.scope),
        }
    }
    fn try_from_optioned(
        value: IngressClassParametersReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.api_group)?,
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
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
            scope: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.scope)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressClassParametersReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.api_group, other.api_group)?;
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
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.scope, other.scope)?;
        Ok(())
    }
}
