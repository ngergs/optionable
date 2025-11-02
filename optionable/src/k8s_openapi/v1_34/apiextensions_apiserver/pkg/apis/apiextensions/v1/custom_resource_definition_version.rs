#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomResourceDefinitionVersionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_printer_columns: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceColumnDefinition,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_warning: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_fields: <Option<
        std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::SelectableField,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub served: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresources: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion {
    type Optioned = CustomResourceDefinitionVersionAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionVersionAc {
    type Optioned = CustomResourceDefinitionVersionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion {
    fn into_optioned(self) -> CustomResourceDefinitionVersionAc {
        CustomResourceDefinitionVersionAc {
            additional_printer_columns: crate::OptionableConvert::into_optioned(
                self.additional_printer_columns,
            ),
            deprecated: crate::OptionableConvert::into_optioned(self.deprecated),
            deprecation_warning: crate::OptionableConvert::into_optioned(
                self.deprecation_warning,
            ),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            schema: crate::OptionableConvert::into_optioned(self.schema),
            selectable_fields: crate::OptionableConvert::into_optioned(
                self.selectable_fields,
            ),
            served: Some(self.served),
            storage: Some(self.storage),
            subresources: crate::OptionableConvert::into_optioned(self.subresources),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionVersionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            additional_printer_columns: crate::OptionableConvert::try_from_optioned(
                value.additional_printer_columns,
            )?,
            deprecated: crate::OptionableConvert::try_from_optioned(value.deprecated)?,
            deprecation_warning: crate::OptionableConvert::try_from_optioned(
                value.deprecation_warning,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            schema: crate::OptionableConvert::try_from_optioned(value.schema)?,
            selectable_fields: crate::OptionableConvert::try_from_optioned(
                value.selectable_fields,
            )?,
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
            subresources: crate::OptionableConvert::try_from_optioned(
                value.subresources,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionVersionAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.additional_printer_columns,
            other.additional_printer_columns,
        )?;
        crate::OptionableConvert::merge(&mut self.deprecated, other.deprecated)?;
        crate::OptionableConvert::merge(
            &mut self.deprecation_warning,
            other.deprecation_warning,
        )?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.schema, other.schema)?;
        crate::OptionableConvert::merge(
            &mut self.selectable_fields,
            other.selectable_fields,
        )?;
        if let Some(other_value) = other.served {
            self.served = other_value;
        }
        if let Some(other_value) = other.storage {
            self.storage = other_value;
        }
        crate::OptionableConvert::merge(&mut self.subresources, other.subresources)?;
        Ok(())
    }
}
