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
            categories: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.categories),
            kind: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kind,
                ),
            ),
            list_kind: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.list_kind),
            plural: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.plural,
                ),
            ),
            short_names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.short_names),
            singular: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.singular),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionNamesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            categories: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.categories)?,
            kind: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            list_kind: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.list_kind)?,
            plural: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .plural
                    .ok_or(crate::optionable::Error {
                        missing_field: "plural",
                    })?,
            )?,
            short_names: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.short_names)?,
            singular: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.singular)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionNamesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.categories, other.categories)?;
        if let Some(other_value) = other.kind {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kind,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.list_kind, other.list_kind)?;
        if let Some(other_value) = other.plural {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.plural,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.short_names, other.short_names)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.singular, other.singular)?;
        Ok(())
    }
}
