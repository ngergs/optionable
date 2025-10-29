pub struct CustomResourceDefinitionSpecOpt {
    pub conversion: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion,
    > as crate::Optionable>::Optioned,
    pub group: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub names: Option<
        <::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::Optionable>::Optioned,
    >,
    pub preserve_unknown_fields: <Option<bool> as crate::Optionable>::Optioned,
    pub scope: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub versions: Option<
        <std::vec::Vec<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionVersion,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    type Optioned = CustomResourceDefinitionSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionSpecOpt {
    type Optioned = CustomResourceDefinitionSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionSpec {
    fn into_optioned(self) -> CustomResourceDefinitionSpecOpt {
        CustomResourceDefinitionSpecOpt {
            conversion: crate::OptionableConvert::into_optioned(self.conversion),
            group: Some(crate::OptionableConvert::into_optioned(self.group)),
            names: Some(crate::OptionableConvert::into_optioned(self.names)),
            preserve_unknown_fields: crate::OptionableConvert::into_optioned(
                self.preserve_unknown_fields,
            ),
            scope: Some(crate::OptionableConvert::into_optioned(self.scope)),
            versions: Some(crate::OptionableConvert::into_optioned(self.versions)),
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conversion: crate::OptionableConvert::try_from_optioned(value.conversion)?,
            group: crate::OptionableConvert::try_from_optioned(
                value
                    .group
                    .ok_or(crate::optionable::Error {
                        missing_field: "group",
                    })?,
            )?,
            names: crate::OptionableConvert::try_from_optioned(
                value
                    .names
                    .ok_or(crate::optionable::Error {
                        missing_field: "names",
                    })?,
            )?,
            preserve_unknown_fields: crate::OptionableConvert::try_from_optioned(
                value.preserve_unknown_fields,
            )?,
            scope: crate::OptionableConvert::try_from_optioned(
                value
                    .scope
                    .ok_or(crate::optionable::Error {
                        missing_field: "scope",
                    })?,
            )?,
            versions: crate::OptionableConvert::try_from_optioned(
                value
                    .versions
                    .ok_or(crate::optionable::Error {
                        missing_field: "versions",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conversion, other.conversion)?;
        if let Some(other_value) = other.group {
            crate::OptionableConvert::merge(&mut self.group, other_value)?;
        }
        if let Some(other_value) = other.names {
            crate::OptionableConvert::merge(&mut self.names, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.preserve_unknown_fields,
            other.preserve_unknown_fields,
        )?;
        if let Some(other_value) = other.scope {
            crate::OptionableConvert::merge(&mut self.scope, other_value)?;
        }
        if let Some(other_value) = other.versions {
            crate::OptionableConvert::merge(&mut self.versions, other_value)?;
        }
        Ok(())
    }
}
