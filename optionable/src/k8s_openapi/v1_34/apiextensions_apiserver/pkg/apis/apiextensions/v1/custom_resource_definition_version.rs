pub struct CustomResourceDefinitionVersionOpt {
    pub additional_printer_columns: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
        >,
    > as crate::Optionable>::Optioned,
    pub deprecated: <Option<bool> as crate::Optionable>::Optioned,
    pub deprecation_warning: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub schema: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
    > as crate::Optionable>::Optioned,
    pub selectable_fields: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
        >,
    > as crate::Optionable>::Optioned,
    pub served: Option<bool>,
    pub storage: Option<bool>,
    pub subresources: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_definition_version::CustomResourceDefinitionVersion {
    type Optioned = CustomResourceDefinitionVersionOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionVersionOpt {
    type Optioned = CustomResourceDefinitionVersionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::custom_resource_definition_version::CustomResourceDefinitionVersion {
    fn into_optioned(self) -> CustomResourceDefinitionVersionOpt {
        CustomResourceDefinitionVersionOpt {
            additional_printer_columns: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
                >,
            > as crate::OptionableConvert>::into_optioned(
                self.additional_printer_columns,
            ),
            deprecated: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.deprecated),
            deprecation_warning: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.deprecation_warning),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            schema: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
            > as crate::OptionableConvert>::into_optioned(self.schema),
            selectable_fields: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
                >,
            > as crate::OptionableConvert>::into_optioned(self.selectable_fields),
            served: Some(self.served),
            storage: Some(self.storage),
            subresources: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
            > as crate::OptionableConvert>::into_optioned(self.subresources),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionVersionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            additional_printer_columns: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
                >,
            > as crate::OptionableConvert>::try_from_optioned(
                value.additional_printer_columns,
            )?,
            deprecated: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.deprecated)?,
            deprecation_warning: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.deprecation_warning,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            schema: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
            > as crate::OptionableConvert>::try_from_optioned(value.schema)?,
            selectable_fields: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.selectable_fields)?,
            served: value
                .served
                .ok_or(crate::optionable::Error {
                    missing_field: "served",
                })?,
            storage: value
                .storage
                .ok_or(crate::optionable::Error {
                    missing_field: "storage",
                })?,
            subresources: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
            > as crate::OptionableConvert>::try_from_optioned(value.subresources)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionVersionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.additional_printer_columns,
            other.additional_printer_columns,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.deprecated, other.deprecated)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.deprecation_warning,
            other.deprecation_warning,
        )?;
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
        > as crate::OptionableConvert>::merge(&mut self.schema, other.schema)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.selectable_fields,
            other.selectable_fields,
        )?;
        if let Some(other_value) = other.served {
            self.served = other_value;
        }
        if let Some(other_value) = other.storage {
            self.storage = other_value;
        }
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
        > as crate::OptionableConvert>::merge(
            &mut self.subresources,
            other.subresources,
        )?;
        Ok(())
    }
}
