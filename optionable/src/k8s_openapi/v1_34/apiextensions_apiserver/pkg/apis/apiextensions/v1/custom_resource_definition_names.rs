pub struct CustomResourceDefinitionNamesOpt {
    pub categories: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub list_kind: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub plural: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub short_names: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub singular: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames {
    type Optioned = CustomResourceDefinitionNamesOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionNamesOpt {
    type Optioned = CustomResourceDefinitionNamesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames {
    fn into_optioned(self) -> CustomResourceDefinitionNamesOpt {
        CustomResourceDefinitionNamesOpt {
            categories: crate::OptionableConvert::into_optioned(self.categories),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            list_kind: crate::OptionableConvert::into_optioned(self.list_kind),
            plural: Some(crate::OptionableConvert::into_optioned(self.plural)),
            short_names: crate::OptionableConvert::into_optioned(self.short_names),
            singular: crate::OptionableConvert::into_optioned(self.singular),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionNamesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            categories: crate::OptionableConvert::try_from_optioned(value.categories)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            list_kind: crate::OptionableConvert::try_from_optioned(value.list_kind)?,
            plural: crate::OptionableConvert::try_from_optioned(
                value
                    .plural
                    .ok_or(crate::optionable::Error {
                        missing_field: "plural",
                    })?,
            )?,
            short_names: crate::OptionableConvert::try_from_optioned(value.short_names)?,
            singular: crate::OptionableConvert::try_from_optioned(value.singular)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionNamesOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.categories, other.categories)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.list_kind, other.list_kind)?;
        if let Some(other_value) = other.plural {
            crate::OptionableConvert::merge(&mut self.plural, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.short_names, other.short_names)?;
        crate::OptionableConvert::merge(&mut self.singular, other.singular)?;
        Ok(())
    }
}
